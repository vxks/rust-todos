use std::io::{self, Error, Stdin};

struct Todo {
    id: i32,
    name: String,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.name)
    }
}

enum Command {
    ADD,
    REMOVE,
    EXIT,
}

impl Command {
    pub fn from_str(string: String) -> Command {
        match string.to_lowercase().trim() {
            "add" => Command::ADD,
            "remove" => Command::REMOVE,
            "exit" => Command::EXIT,
            s => panic!("Illegal command: [{}]", s)
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::ADD => write!(f, "ADD"),
            Command::REMOVE => write!(f, "REMOVE"),
            Command::EXIT => write!(f, "EXIT")
        }
    }
}


fn print_todos(todos: &[Todo]) {
    println!("Todos:");
    for todo in todos {
        println!("- {}", todo.to_string());
    }
}

fn prompt_add_todo(todos: &mut Vec<Todo>) {
    let stdin: Stdin = io::stdin();
    println!("Enter todo: ");
    let mut todo_in_buf = String::new();
    stdin.read_line(&mut todo_in_buf).unwrap();
    let id = todos.last().map_or(0, |todo| todo.id + 1);
    let todo = Todo { id, name: todo_in_buf.trim().to_string() };
    todos.push(todo);
}

fn prompt_remove_todo(todos: &mut Vec<Todo>) {
    let stdin: Stdin = io::stdin();
    println!("Todo ID to remove: ");
    let mut todo_id_in_buf = String::new();
    stdin.read_line(&mut todo_id_in_buf).unwrap();
    let idx = todos
        .into_iter()
        .position(|todo| todo.id == todo_id_in_buf.trim().parse().unwrap());
    todos.remove(idx.unwrap());
}

fn get_command() -> Command {
    let stdin: Stdin = io::stdin();
    let mut in_buf = String::new();
    println!("Enter command: ");
    stdin.read_line(&mut in_buf).unwrap();
    Command::from_str(in_buf)
}

fn print_commands() {
    println!("Available commands:");
    println!(" \"add\"");
    println!(" \"remove\"");
    println!(" \"exit\"");
}

fn main() -> Result<(), Error> {
    let mut todos: Vec<Todo> = vec![];

    loop {
        print_commands();
        match get_command() {
            Command::ADD => prompt_add_todo(&mut todos),
            Command::REMOVE => prompt_remove_todo(&mut todos),
            Command::EXIT => std::process::exit(1),
        };
        print_todos(&todos);
    }
}
