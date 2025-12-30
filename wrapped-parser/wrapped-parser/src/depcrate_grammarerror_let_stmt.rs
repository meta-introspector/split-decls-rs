// Generated macro for error_let_stmt (function)
macro_rules! Depcrate_grammarerror_let_stmt {
() => {
// Module: crate::grammar
// Provides: {"error_let_stmt"}
// Dependencies: {}
fn error_let_stmt (p : & mut Parser < '_ > , message : & str) { assert ! (p . at (T ! [let])) ; let m = p . start () ; p . error (message) ; expressions :: let_stmt (p , expressions :: Semicolon :: Optional) ; m . complete (p , ERROR) ; }
};
}
