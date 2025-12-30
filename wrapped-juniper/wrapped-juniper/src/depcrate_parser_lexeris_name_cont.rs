// Generated macro for is_name_cont (function)
macro_rules! Depcrate_parser_lexeris_name_cont {
() => {
// Module: crate::parser::lexer
// Provides: {"is_name_cont"}
// Dependencies: {}
fn is_name_cont (c : char) -> bool { is_name_start (c) || c . is_ascii_digit () }
};
}
