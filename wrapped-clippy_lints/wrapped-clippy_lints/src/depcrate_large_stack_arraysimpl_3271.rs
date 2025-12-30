// Generated macro for impl_3271 (impl)
macro_rules! Depcrate_large_stack_arraysimpl_3271 {
() => {
// Module: crate::large_stack_arrays
// Provides: {"impl_3271"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for LargeStackArrays { fn check_item (& mut self , _ : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if matches ! (item . kind , ItemKind :: Static (..) | ItemKind :: Const (..)) { self . const_item_counter += 1 ; } } fn check_item_post (& mut self , _ : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if matches ! (item . kind , ItemKind :: Static (..) | ItemKind :: Const (..)) { self . const_item_counter -= 1 ; } } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if self . const_item_counter . 0 == 0 && let ExprKind :: Repeat (_ , _) | ExprKind :: Array (_) = expr . kind && ! self . is_from_vec_macro (cx , expr . span) && let ty :: Array (element_type , cst) = cx . typeck_results () . expr_ty (expr) . kind () && let Some (element_count) = cst . try_to_target_usize (cx . tcx) && let Ok (element_size) = cx . layout_of (* element_type) . map (| l | l . size . bytes ()) && ! cx . tcx . hir_parent_iter (expr . hir_id) . any (| (_ , node) | { matches ! (node , Node :: Item (Item { kind : ItemKind :: Static (..) , .. })) }) && u128 :: from (self . maximum_allowed_size) < u128 :: from (element_count) * u128 :: from (element_size) { span_lint_and_then (cx , LARGE_STACK_ARRAYS , expr . span , format ! ("allocating a local array larger than {} bytes" , self . maximum_allowed_size) , | diag | { if ! might_be_expanded (cx , expr) { diag . help (format ! ("consider allocating on the heap with `vec!{}.into_boxed_slice()`" , snippet (cx , expr . span , "[...]"))) ; } } ,) ; } } }
};
}
