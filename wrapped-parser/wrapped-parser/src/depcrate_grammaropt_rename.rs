// Generated macro for opt_rename (function)
macro_rules! Depcrate_grammaropt_rename {
() => {
// Module: crate::grammar
// Provides: {"opt_rename"}
// Dependencies: {}
fn opt_rename (p : & mut Parser < '_ >) { if p . at (T ! [as]) { let m = p . start () ; p . bump (T ! [as]) ; if ! p . eat (T ! [_]) { name (p) ; } m . complete (p , RENAME) ; } }
};
}
