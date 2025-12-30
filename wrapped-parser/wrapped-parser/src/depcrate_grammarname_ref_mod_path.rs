// Generated macro for name_ref_mod_path (function)
macro_rules! Depcrate_grammarname_ref_mod_path {
() => {
// Module: crate::grammar
// Provides: {"name_ref_mod_path"}
// Dependencies: {}
fn name_ref_mod_path (p : & mut Parser < '_ >) { if p . at_ts (PATH_NAME_REF_KINDS) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected identifier, `self`, `super`, `crate`, or `Self`") ; } }
};
}
