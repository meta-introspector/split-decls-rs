// Generated macro for has_attr (function)
macro_rules! Depcrate_utils_dump_hirhas_attr {
() => {
// Module: crate::utils::dump_hir
// Provides: {"has_attr"}
// Dependencies: {}
fn has_attr (cx : & LateContext < '_ > , hir_id : hir :: HirId) -> bool { let attrs = cx . tcx . hir_attrs (hir_id) ; get_builtin_attr (cx . sess () , attrs , sym :: dump) . count () > 0 }
};
}
