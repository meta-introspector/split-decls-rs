// Generated macro for error_block (function)
macro_rules! Depcrate_grammarerror_block {
() => {
// Module: crate::grammar
// Provides: {"error_block"}
// Dependencies: {}
fn error_block (p : & mut Parser < '_ > , message : & str) { assert ! (p . at (T ! ['{'])) ; let m = p . start () ; p . error (message) ; p . bump (T ! ['{']) ; expressions :: expr_block_contents (p) ; p . eat (T ! ['}']) ; m . complete (p , ERROR) ; }
};
}
