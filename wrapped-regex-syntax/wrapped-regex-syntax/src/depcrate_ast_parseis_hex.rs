// Generated macro for is_hex (function)
macro_rules! Depcrate_ast_parseis_hex {
() => {
// Module: crate::ast::parse
// Provides: {"is_hex"}
// Dependencies: {}
# [doc = " Returns true if the given character is a hexadecimal digit."] fn is_hex (c : char) -> bool { ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F') }
};
}
