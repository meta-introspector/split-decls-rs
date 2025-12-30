// Generated macro for is_ptr_to_ref (function)
macro_rules! Depcrate_swap_ptr_to_refis_ptr_to_ref {
() => {
// Module: crate::swap_ptr_to_ref
// Provides: {"is_ptr_to_ref"}
// Dependencies: {}
# [doc = " Checks if the expression converts a mutable pointer to a mutable reference. If it is, also"] # [doc = " returns the span of the pointer expression if it's suitable for making a suggestion."] fn is_ptr_to_ref (cx : & LateContext < '_ > , e : & Expr < '_ > , ctxt : SyntaxContext) -> (bool , Option < Span >) { if let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Mut , borrowed_expr) = e . kind && let ExprKind :: Unary (UnOp :: Deref , derefed_expr) = borrowed_expr . kind && cx . typeck_results () . expr_ty (derefed_expr) . is_raw_ptr () { (true , (borrowed_expr . span . ctxt () == ctxt || derefed_expr . span . ctxt () == ctxt) . then_some (derefed_expr . span) ,) } else { (false , None) } }
};
}
