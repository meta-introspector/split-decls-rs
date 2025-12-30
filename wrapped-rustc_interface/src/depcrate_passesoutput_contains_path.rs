// Generated macro for output_contains_path (function)
macro_rules! Depcrate_passesoutput_contains_path {
() => {
// Module: crate::passes
// Provides: {"output_contains_path"}
// Dependencies: {}
fn output_contains_path (output_paths : & [PathBuf] , input_path : & Path) -> bool { let input_path = try_canonicalize (input_path) . ok () ; if input_path . is_none () { return false ; } output_paths . iter () . any (| output_path | try_canonicalize (output_path) . ok () == input_path) }
};
}
