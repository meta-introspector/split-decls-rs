// Generated macro for expect_ident (function)
macro_rules! Depcrate_parseexpect_ident {
() => {
// Module: crate::parse
// Provides: {"expect_ident"}
// Dependencies: {}
# [doc = " Coerce an expression into a simple identifier."] fn expect_ident (expr : Expr) -> syn :: Result < Ident > { syn :: parse2 (expr . into_token_stream ()) }
};
}
