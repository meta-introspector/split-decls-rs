// Generated macro for name_r (function)
macro_rules! Depcrate_grammarname_r {
() => {
// Module: crate::grammar
// Provides: {"name_r"}
// Dependencies: {}
fn name_r (p : & mut Parser < '_ > , recovery : TokenSet) { if p . at (IDENT) { let m = p . start () ; p . bump (IDENT) ; m . complete (p , NAME) ; } else { p . err_recover ("expected a name" , recovery) ; } }
};
}
