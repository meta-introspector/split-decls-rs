// Generated macro for expect_litbool (function)
macro_rules! Depcrate_parseexpect_litbool {
() => {
// Module: crate::parse
// Provides: {"expect_litbool"}
// Dependencies: {}
# [doc = " Coerce an expression into a simple keyword."] fn expect_litbool (expr : Expr) -> syn :: Result < LitBool > { syn :: parse2 (expr . into_token_stream ()) }
};
}
