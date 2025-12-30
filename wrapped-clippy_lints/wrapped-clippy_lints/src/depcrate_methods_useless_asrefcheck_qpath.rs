// Generated macro for check_qpath (function)
macro_rules! Depcrate_methods_useless_asrefcheck_qpath {
() => {
// Module: crate::methods::useless_asref
// Provides: {"check_qpath"}
// Dependencies: {}
fn check_qpath (cx : & LateContext < '_ > , qpath : hir :: QPath < '_ > , hir_id : hir :: HirId) -> bool { if let Some (path_def_id) = cx . qpath_res (& qpath , hir_id) . opt_def_id () { cx . tcx . lang_items () . get (LangItem :: CloneFn) == Some (path_def_id) } else { false } }
};
}
