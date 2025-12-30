// Generated macro for parse_config (function)
macro_rules! Depcrateparse_config {
() => {
// Module: crate
// Provides: {"parse_config"}
// Dependencies: {}
fn parse_config (args : & [String]) -> Config { let query = args [1] . clone () ; let file_path = args [2] . clone () ; Config { query , file_path } }
};
}
