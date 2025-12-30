// Generated macro for expr_as_ptr_offset_call (function)
macro_rules! Depcrate_ptr_offset_with_castexpr_as_ptr_offset_call {
() => {
// Module: crate::ptr_offset_with_cast
// Provides: {"expr_as_ptr_offset_call"}
// Dependencies: {}
fn expr_as_ptr_offset_call < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > ,) -> Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx > , Method) > { if let ExprKind :: MethodCall (path_segment , arg_0 , [arg_1] , _) = & expr . kind && is_expr_ty_raw_ptr (cx , arg_0) { if path_segment . ident . name == sym :: offset { return Some ((arg_0 , arg_1 , Method :: Offset)) ; } if path_segment . ident . name == sym :: wrapping_offset { return Some ((arg_0 , arg_1 , Method :: WrappingOffset)) ; } } None }
};
}
