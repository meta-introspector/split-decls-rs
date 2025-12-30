// Generated macro for mark_block_as_reachable (function)
macro_rules! Depcrate_loops_never_loopmark_block_as_reachable {
() => {
// Module: crate::loops::never_loop
// Provides: {"mark_block_as_reachable"}
// Dependencies: {}
fn mark_block_as_reachable (expr : & Expr < '_ > , local_labels : & mut [(HirId , bool)]) { if let ExprKind :: Break (Destination { target_id : Ok (t) , .. } , _) = expr . kind && let Some ((_ , reachable)) = local_labels . iter_mut () . find (| (label , _) | * label == t) { * reachable = true ; } }
};
}
