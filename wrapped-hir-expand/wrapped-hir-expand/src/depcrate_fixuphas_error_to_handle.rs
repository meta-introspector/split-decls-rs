// Generated macro for has_error_to_handle (function)
macro_rules! Depcrate_fixuphas_error_to_handle {
() => {
// Module: crate::fixup
// Provides: {"has_error_to_handle"}
// Dependencies: {}
fn has_error_to_handle (node : & SyntaxNode) -> bool { has_error (node) || node . children () . any (| c | ! can_handle_error (& c) && has_error_to_handle (& c)) }
};
}
