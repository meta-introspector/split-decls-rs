// Generated macro for name_ref_or_upper_self (function)
macro_rules! Depcrate_grammarname_ref_or_upper_self {
() => {
// Module: crate::grammar
// Provides: {"name_ref_or_upper_self"}
// Dependencies: {}
fn name_ref_or_upper_self (p : & mut Parser < '_ >) { if matches ! (p . current () , T ! [ident] | T ! [Self]) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected identifier or `Self`") ; } }
};
}
