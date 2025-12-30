// Generated macro for print_paths (function)
macro_rules! Depcrate_core_build_steps_formatprint_paths {
() => {
// Module: crate::core::build_steps::format
// Provides: {"print_paths"}
// Dependencies: {}
fn print_paths (verb : & str , adjective : Option < & str > , paths : & [String]) { let len = paths . len () ; let adjective = if let Some (adjective) = adjective { format ! ("{adjective} ") } else { String :: new () } ; if len <= 10 { for path in paths { println ! ("fmt: {verb} {adjective}file {path}") ; } } else { println ! ("fmt: {verb} {len} {adjective}files") ; } }
};
}
