// Generated macro for is_token (function)
macro_rules! Depcrate_crate_in_macro_defis_token {
() => {
// Module: crate::crate_in_macro_def
// Provides: {"is_token"}
// Dependencies: {}
fn is_token (tt : & TokenTree , kind : & TokenKind) -> bool { if let TokenTree :: Token (Token { kind : other , .. } , _) = tt { kind == other } else { false } }
};
}
