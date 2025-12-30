// Generated macro for parse_file (function)
macro_rules! Depcrateparse_file {
() => {
// Module: crate
// Provides: {"parse_file"}
// Dependencies: {}
fn parse_file (path : & str) -> Result < File > { let content = fs :: read_to_string (path) . with_context (| | format ! ("Failed to read file: {}" , path)) ? ; syn :: parse_file (& content) . with_context (| | format ! ("Failed to parse Rust file: {}" , path)) }
};
}
