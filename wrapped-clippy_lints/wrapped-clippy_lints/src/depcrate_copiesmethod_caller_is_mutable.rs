// Generated macro for method_caller_is_mutable (function)
macro_rules! Depcrate_copiesmethod_caller_is_mutable {
() => {
// Module: crate::copies
// Provides: {"method_caller_is_mutable"}
// Dependencies: {}
fn method_caller_is_mutable < 'tcx > (cx : & LateContext < 'tcx > , caller_expr : & Expr < '_ > , interior_mut : & mut InteriorMut < 'tcx > ,) -> bool { let caller_ty = cx . typeck_results () . expr_ty (caller_expr) ; interior_mut . is_interior_mut_ty (cx , caller_ty) || caller_ty . is_mutable_ptr () || path_to_local (caller_expr) . and_then (| hid | find_binding_init (cx , hid)) . is_none () }
};
}
