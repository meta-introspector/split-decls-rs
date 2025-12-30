// Generated macro for check_partial_eq (function)
macro_rules! Depcrate_unconditional_recursioncheck_partial_eq {
() => {
// Module: crate::unconditional_recursion
// Provides: {"check_partial_eq"}
// Dependencies: {}
fn check_partial_eq (cx : & LateContext < '_ > , method_span : Span , method_def_id : LocalDefId , name : Ident , expr : & Expr < '_ >) { let Some (sig) = cx . typeck_results () . liberated_fn_sigs () . get (cx . tcx . local_def_id_to_hir_id (method_def_id)) else { return ; } ; if let [self_arg , other_arg] = sig . inputs () && let & ty :: Ref (_ , self_arg , _) = self_arg . kind () && let & ty :: Ref (_ , other_arg , _) = other_arg . kind () && let Some (trait_def_id) = get_impl_trait_def_id (cx , method_def_id) && cx . tcx . is_diagnostic_item (sym :: PartialEq , trait_def_id) { let to_check_op = if name . name == sym :: eq { BinOpKind :: Eq } else { BinOpKind :: Ne } ; let is_bad = match expr . kind { ExprKind :: Binary (op , left , right) if op . node == to_check_op => { let left_ty = cx . typeck_results () . expr_ty_adjusted (left) ; let right_ty = cx . typeck_results () . expr_ty_adjusted (right) ; matches_ty (left_ty , right_ty , self_arg , other_arg) } , ExprKind :: MethodCall (segment , receiver , [arg] , _) if segment . ident . name == name . name => { let receiver_ty = cx . typeck_results () . expr_ty_adjusted (receiver) ; let arg_ty = cx . typeck_results () . expr_ty_adjusted (arg) ; if let Some (fn_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let Some (trait_id) = cx . tcx . trait_of_assoc (fn_id) && trait_id == trait_def_id && matches_ty (receiver_ty , arg_ty , self_arg , other_arg) { true } else { false } } , _ => false , } ; if is_bad { span_error (cx , method_span , expr) ; } } }
};
}
