// Generated macro for get_pointee_ty_and_count_expr (function)
macro_rules! Depcrate_size_of_in_element_countget_pointee_ty_and_count_expr {
() => {
// Module: crate::size_of_in_element_count
// Provides: {"get_pointee_ty_and_count_expr"}
// Dependencies: {}
fn get_pointee_ty_and_count_expr < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > ,) -> Option < (Ty < 'tcx > , & 'tcx Expr < 'tcx >) > { const METHODS : [Symbol ; 10] = [sym :: copy_to , sym :: copy_from , sym :: copy_to_nonoverlapping , sym :: copy_from_nonoverlapping , sym :: add , sym :: wrapping_add , sym :: sub , sym :: wrapping_sub , sym :: offset , sym :: wrapping_offset ,] ; if let ExprKind :: Call (func , [.. , count]) = expr . kind && let ExprKind :: Path (ref func_qpath) = func . kind && let Some (def_id) = cx . qpath_res (func_qpath , func . hir_id) . opt_def_id () && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_copy | sym :: ptr_copy_nonoverlapping | sym :: ptr_slice_from_raw_parts | sym :: ptr_slice_from_raw_parts_mut | sym :: ptr_swap_nonoverlapping | sym :: slice_from_raw_parts | sym :: slice_from_raw_parts_mut)) && let Some (pointee_ty) = cx . typeck_results () . node_args (func . hir_id) . types () . next () { return Some ((pointee_ty , count)) ; } if let ExprKind :: MethodCall (method_path , ptr_self , [.. , count] , _) = expr . kind && let method_ident = method_path . ident . name && METHODS . contains (& method_ident) && let ty :: RawPtr (pointee_ty , _) = cx . typeck_results () . expr_ty (ptr_self) . kind () { return Some ((* pointee_ty , count)) ; } None }
};
}
