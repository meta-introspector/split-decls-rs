// Generated macro for has_attr (function)
macro_rules! Depcrate_utils_authorhas_attr {
() => {
// Module: crate::utils::author
// Provides: {"has_attr"}
// Dependencies: {}
fn has_attr (cx : & LateContext < '_ > , hir_id : HirId) -> bool { let attrs = cx . tcx . hir_attrs (hir_id) ; get_builtin_attr (cx . sess () , attrs , sym :: author) . count () > 0 }
};
}
