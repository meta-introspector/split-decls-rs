// Generated macro for impl_3765 (impl)
macro_rules! Depcrate_loops_same_item_pushimpl_3765 {
() => {
// Module: crate::loops::same_item_push
// Provides: {"impl_3765"}
// Dependencies: {}
impl < 'a , 'tcx > SameItemPushVisitor < 'a , 'tcx > { fn new (cx : & 'a LateContext < 'tcx >) -> Self { Self { non_deterministic_expr : false , multiple_pushes : false , vec_push : None , cx , used_locals : FxHashSet :: default () , } } fn should_lint (& self) -> bool { if ! self . non_deterministic_expr && ! self . multiple_pushes && let Some ((vec , _ , _)) = self . vec_push && let Some (hir_id) = path_to_local (vec) { ! self . used_locals . contains (& hir_id) } else { false } } }
};
}
