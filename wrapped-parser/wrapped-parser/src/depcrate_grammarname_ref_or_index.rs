// Generated macro for name_ref_or_index (function)
macro_rules! Depcrate_grammarname_ref_or_index {
() => {
// Module: crate::grammar
// Provides: {"name_ref_or_index"}
// Dependencies: {}
fn name_ref_or_index (p : & mut Parser < '_ >) { assert ! (p . at (IDENT) || p . at (INT_NUMBER)) ; let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; }
};
}
