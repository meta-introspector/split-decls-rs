// Generated macro for check (function)
macro_rules! Depcrate_loops_explicit_into_iter_loopcheck {
() => {
// Module: crate::loops::explicit_into_iter_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , self_arg : & Expr < '_ > , call_expr : & Expr < '_ >) { if ! cx . ty_based_def (call_expr) . opt_parent (cx) . is_diag_item (cx , sym :: IntoIterator) { return ; } let typeck = cx . typeck_results () ; let self_ty = typeck . expr_ty (self_arg) ; let adjust = match typeck . expr_adjustments (self_arg) { [] => AdjustKind :: None , & [Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) , .. } ,] => AdjustKind :: borrow (mutbl) , & [Adjustment { kind : Adjust :: Deref (_) , .. } , Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) , target , } ,] => { if self_ty == target && matches ! (mutbl , AutoBorrowMutability :: Not) { AdjustKind :: None } else { AdjustKind :: reborrow (mutbl) } } , _ => return , } ; let mut applicability = Applicability :: MachineApplicable ; let object = snippet_with_context (cx , self_arg . span , call_expr . span . ctxt () , "_" , & mut applicability) . 0 ; span_lint_and_sugg (cx , EXPLICIT_INTO_ITER_LOOP , call_expr . span , "it is more concise to loop over containers instead of using explicit \
            iteration methods" , "to write this more concisely, try" , format ! ("{}{object}" , adjust . display ()) , applicability ,) ; }
};
}
