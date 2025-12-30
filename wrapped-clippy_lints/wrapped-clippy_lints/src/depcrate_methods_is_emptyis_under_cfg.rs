// Generated macro for is_under_cfg (function)
macro_rules! Depcrate_methods_is_emptyis_under_cfg {
() => {
// Module: crate::methods::is_empty
// Provides: {"is_under_cfg"}
// Dependencies: {}
fn is_under_cfg (cx : & LateContext < '_ > , id : HirId) -> bool { cx . tcx . hir_parent_id_iter (id) . any (| id | cx . tcx . hir_attrs (id) . iter () . any (| attr | attr . has_name (sym :: cfg_trace))) }
};
}
