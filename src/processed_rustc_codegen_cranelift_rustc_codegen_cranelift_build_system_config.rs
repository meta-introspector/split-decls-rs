// SRC: ../rust/compiler/rustc_codegen_cranelift/build_system/config.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::{fs, process};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=load_config_file | COMPLEXITY=12 | LINES=17 */

fn load_config_file() -> Vec<(String, Option<String>)> {
    fs::read_to_string("config.txt")
        .unwrap()
        .lines()
        .map(|line| if let Some((line, _comment)) = line.split_once('#') { line } else { line })
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let Some((key, val)) = line.split_once('=') {
                (key.trim().to_owned(), Some(val.trim().to_owned()))
            } else {
                (line.to_owned(), None)
            }
        })
        .collect()
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=17 */

pub(crate) fn get_bool(name: &str) -> bool {
    let values = load_config_file()
        .into_iter()
        .filter(|(key, _)| key == name)
        .map(|(_, val)| val)
        .collect::<Vec<_>>();
    if values.is_empty() {
        false
    } else {
        if values.iter().any(|val| val.is_some()) {
            eprintln!("Boolean config `{}` has a value", name);
            process::exit(1);
        }
        true
    }
}