// Generated macro for check_implicit_cast (function)
macro_rules! Depcrate_casts_borrow_as_ptrcheck_implicit_cast {
() => {
// Module: crate::casts::borrow_as_ptr
// Provides: {"check_implicit_cast"}
// Dependencies: {}
# [doc = " Check for an implicit cast from reference to raw pointer outside an explicit `as`."] pub (super) fn check_implicit_cast (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if ! expr . span . from_expansion () && let ExprKind :: AddrOf (BorrowKind :: Ref , _ , pointee) = expr . kind && ! matches ! (get_parent_expr (cx , expr) . map (| e | e . kind) , Some (ExprKind :: Cast (..))) && let [deref , borrow] = cx . typeck_results () . expr_adjustments (expr) && matches ! (deref . kind , Adjust :: Deref (..)) && let Adjust :: Borrow (AutoBorrow :: RawPtr (mutability)) = borrow . kind && ! is_expr_temporary_value (cx , pointee) { span_lint_and_then (cx , BORROW_AS_PTR , expr . span , "implicit borrow as raw pointer" , | diag | { diag . span_suggestion_verbose (expr . span . until (pointee . span) , "use a raw pointer instead" , format ! ("&raw {} " , mutability . ptr_str ()) , Applicability :: MachineApplicable ,) ; }) ; } }
};
}
