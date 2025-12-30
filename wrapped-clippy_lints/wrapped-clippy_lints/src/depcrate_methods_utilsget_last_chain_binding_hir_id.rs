// Generated macro for get_last_chain_binding_hir_id (function)
macro_rules! Depcrate_methods_utilsget_last_chain_binding_hir_id {
() => {
// Module: crate::methods::utils
// Provides: {"get_last_chain_binding_hir_id"}
// Dependencies: {}
pub (super) fn get_last_chain_binding_hir_id (mut hir_id : HirId , statements : & [Stmt < '_ >]) -> Option < HirId > { for stmt in statements { if let StmtKind :: Let (local) = stmt . kind && let Some (init) = local . init && let ExprKind :: Path (QPath :: Resolved (_ , path)) = init . kind && let rustc_hir :: def :: Res :: Local (local_hir_id) = path . res && local_hir_id == hir_id { hir_id = local . pat . hir_id ; } else { return None ; } } Some (hir_id) }
};
}
