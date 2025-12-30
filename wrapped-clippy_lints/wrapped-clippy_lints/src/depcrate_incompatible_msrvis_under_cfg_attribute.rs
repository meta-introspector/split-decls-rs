// Generated macro for is_under_cfg_attribute (function)
macro_rules! Depcrate_incompatible_msrvis_under_cfg_attribute {
() => {
// Module: crate::incompatible_msrv
// Provides: {"is_under_cfg_attribute"}
// Dependencies: {}
# [doc = " Heuristic checking if the node `hir_id` is under a `#[cfg()]` or `#[cfg_attr()]`"] # [doc = " attribute."] fn is_under_cfg_attribute (cx : & LateContext < '_ > , hir_id : HirId) -> bool { cx . tcx . hir_parent_id_iter (hir_id) . any (| id | { cx . tcx . hir_attrs (id) . iter () . any (| attr | { matches ! (attr . ident () . map (| ident | ident . name) , Some (sym :: cfg_trace | sym :: cfg_attr_trace)) }) }) }
};
}
