// Generated macro for is_align_of_call (function)
macro_rules! Depcrate_casts_manual_dangling_ptris_align_of_call {
() => {
// Module: crate::casts::manual_dangling_ptr
// Provides: {"is_align_of_call"}
// Dependencies: {}
fn is_align_of_call (cx : & LateContext < '_ > , fun : & Expr < '_ > , to : & Ty < '_ >) -> bool { if let ExprKind :: Path (QPath :: Resolved (_ , path)) = fun . kind && fun . basic_res () . is_diag_item (cx , sym :: mem_align_of) && let Some (args) = path . segments . last () . and_then (| seg | seg . args) && let [GenericArg :: Type (generic_ty)] = args . args { let typeck = cx . typeck_results () ; return typeck . node_type (generic_ty . hir_id) == typeck . node_type (to . hir_id) ; } false }
};
}
