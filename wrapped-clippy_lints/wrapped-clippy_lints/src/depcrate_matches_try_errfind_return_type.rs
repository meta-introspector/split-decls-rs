// Generated macro for find_return_type (function)
macro_rules! Depcrate_matches_try_errfind_return_type {
() => {
// Module: crate::matches::try_err
// Provides: {"find_return_type"}
// Dependencies: {}
# [doc = " Finds function return type by examining return expressions in match arms."] fn find_return_type < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx ExprKind < '_ >) -> Option < Ty < 'tcx > > { if let ExprKind :: Match (_ , arms , MatchSource :: TryDesugar (_)) = expr { for arm in * arms { if let ExprKind :: Ret (Some (ret)) = arm . body . kind { return Some (cx . typeck_results () . expr_ty (ret)) ; } } } None }
};
}
