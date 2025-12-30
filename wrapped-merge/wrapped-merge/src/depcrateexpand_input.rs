// Generated macro for expand_input (function)
macro_rules! Depcrateexpand_input {
() => {
// Module: crate
// Provides: {"expand_input"}
// Dependencies: {}
fn expand_input (input : Vec < String >) -> Vec < reader :: File > { let mut result = vec ! [] ; let read_file = | path | { reader :: File :: read (& path) . unwrap_or_else (| | panic ! ("failed to read .winmd format `{path}`")) } ; for input in input { let path = std :: path :: Path :: new (& input) ; if path . is_dir () { let prev_len = result . len () ; for path in path . read_dir () . unwrap_or_else (| _ | panic ! ("failed to read directory `{input}`")) . flatten () . map (| entry | entry . path ()) { if path . is_file () && path . extension () . is_some_and (| extension | extension . eq_ignore_ascii_case ("winmd")) { result . push (read_file (path . to_string_lossy () . to_string ())) ; } } if result . len () == prev_len { panic ! ("failed to find .winmd files in directory `{input}`") ; } } else { result . push (read_file (input)) ; } } result }
};
}
