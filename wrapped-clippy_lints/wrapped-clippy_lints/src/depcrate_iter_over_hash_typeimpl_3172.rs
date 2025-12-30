// Generated macro for impl_3172 (impl)
macro_rules! Depcrate_iter_over_hash_typeimpl_3172 {
() => {
// Module: crate::iter_over_hash_type
// Provides: {"impl_3172"}
// Dependencies: {}
impl LateLintPass < '_ > for IterOverHashType { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ rustc_hir :: Expr < '_ >) { let hash_iter_tys = [sym :: HashMap , sym :: HashSet , sym :: hashmap_keys_ty , sym :: hashmap_values_ty , sym :: hashmap_values_mut_ty , sym :: hashmap_iter_ty , sym :: hashmap_iter_mut_ty , sym :: hashmap_drain_ty , sym :: hashset_iter_ty , sym :: hashset_drain_ty ,] ; if let Some (for_loop) = ForLoop :: hir (expr) && ! for_loop . body . span . from_expansion () && let ty = cx . typeck_results () . expr_ty (for_loop . arg) . peel_refs () && hash_iter_tys . into_iter () . any (| sym | ty . is_diag_item (cx , sym)) { span_lint (cx , ITER_OVER_HASH_TYPE , expr . span , "iteration over unordered hash-based type" ,) ; } } }
};
}
