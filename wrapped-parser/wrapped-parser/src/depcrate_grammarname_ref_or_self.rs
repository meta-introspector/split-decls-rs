// Generated macro for name_ref_or_self (function)
macro_rules! Depcrate_grammarname_ref_or_self {
() => {
// Module: crate::grammar
// Provides: {"name_ref_or_self"}
// Dependencies: {}
fn name_ref_or_self (p : & mut Parser < '_ >) { if matches ! (p . current () , T ! [ident] | T ! [self]) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected identifier or `self`") ; } }
};
}
