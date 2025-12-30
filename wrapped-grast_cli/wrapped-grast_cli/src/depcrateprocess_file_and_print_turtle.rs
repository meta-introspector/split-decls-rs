// Generated macro for process_file_and_print_turtle (function)
macro_rules! Depcrateprocess_file_and_print_turtle {
() => {
// Module: crate
// Provides: {"process_file_and_print_turtle"}
// Dependencies: {}
fn process_file_and_print_turtle (file_path : & Path) -> Result < () > { let code = fs :: read_to_string (file_path) . context (format ! ("Failed to read Rust file: {}" , file_path . display ())) ? ; let ast = parse_file (& code) . context (format ! ("Failed to parse Rust code from: {}" , file_path . display ())) ? ; let mut db = GrastDb :: new () ; db . flatten (& ast) ; print ! ("{}" , db . to_turtle ()) ; Ok (()) }
};
}
