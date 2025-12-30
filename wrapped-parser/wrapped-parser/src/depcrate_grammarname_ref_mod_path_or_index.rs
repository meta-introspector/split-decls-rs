// Generated macro for name_ref_mod_path_or_index (function)
macro_rules! Depcrate_grammarname_ref_mod_path_or_index {
() => {
// Module: crate::grammar
// Provides: {"name_ref_mod_path_or_index"}
// Dependencies: {}
fn name_ref_mod_path_or_index (p : & mut Parser < '_ >) { if p . at_ts (PATH_NAME_REF_OR_INDEX_KINDS) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected integer, identifier, `self`, `super`, `crate`, or `Self`") ; } }
};
}
