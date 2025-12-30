// Generated macro for check (function)
macro_rules! Depcrate_operators_manual_is_multiple_ofcheck {
() => {
// Module: crate::operators::manual_is_multiple_of
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , op : BinOpKind , lhs : & 'tcx Expr < 'tcx > , rhs : & 'tcx Expr < 'tcx > , msrv : Msrv ,) { if msrv . meets (cx , msrvs :: UNSIGNED_IS_MULTIPLE_OF) && let Some (operand) = uint_compare_to_zero (cx , expr , op , lhs , rhs) && let ExprKind :: Binary (operand_op , operand_left , operand_right) = operand . kind && operand_op . node == BinOpKind :: Rem && matches ! (cx . typeck_results () . expr_ty_adjusted (operand_left) . peel_refs () . kind () , ty :: Uint (_)) && matches ! (cx . typeck_results () . expr_ty_adjusted (operand_right) . peel_refs () . kind () , ty :: Uint (_)) && expr_type_is_certain (cx , operand_left) { let mut app = Applicability :: MachineApplicable ; let divisor = deref_sugg (Sugg :: hir_with_applicability (cx , operand_right , "_" , & mut app) , cx . typeck_results () . expr_ty_adjusted (operand_right) ,) ; span_lint_and_sugg (cx , MANUAL_IS_MULTIPLE_OF , expr . span , "manual implementation of `.is_multiple_of()`" , "replace with" , format ! ("{}{}.is_multiple_of({divisor})" , if op == BinOpKind :: Eq { "" } else { "!" } , Sugg :: hir_with_applicability (cx , operand_left , "_" , & mut app) . maybe_paren ()) , app ,) ; } }
};
}
