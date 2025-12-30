// Generated macro for lifetime (function)
macro_rules! Depcrate_grammarlifetime {
() => {
// Module: crate::grammar
// Provides: {"lifetime"}
// Dependencies: {}
fn lifetime (p : & mut Parser < '_ >) { assert ! (p . at (LIFETIME_IDENT)) ; let m = p . start () ; p . bump (LIFETIME_IDENT) ; m . complete (p , LIFETIME) ; }
};
}
