// Generated macro for detect_absurd_comparison (function)
macro_rules! Depcrate_operators_absurd_extreme_comparisonsdetect_absurd_comparison {
() => {
// Module: crate::operators::absurd_extreme_comparisons
// Provides: {"detect_absurd_comparison"}
// Dependencies: {}
fn detect_absurd_comparison < 'tcx > (cx : & LateContext < 'tcx > , op : BinOpKind , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > ,) -> Option < (ExtremeExpr < 'tcx > , AbsurdComparisonResult) > { use AbsurdComparisonResult :: { AlwaysFalse , AlwaysTrue , InequalityImpossible } ; use ExtremeType :: { Maximum , Minimum } ; if cx . typeck_results () . expr_ty (lhs) != cx . typeck_results () . expr_ty (rhs) { return None ; } if is_cast_between_fixed_and_target (cx , lhs) || is_cast_between_fixed_and_target (cx , rhs) { return None ; } let (rel , normalized_lhs , normalized_rhs) = normalize_comparison (op , lhs , rhs) ? ; let lx = detect_extreme_expr (cx , normalized_lhs) ; let rx = detect_extreme_expr (cx , normalized_rhs) ; Some (match rel { Rel :: Lt => { match (lx , rx) { (Some (l @ ExtremeExpr { which : Maximum , .. }) , _) => (l , AlwaysFalse) , (_ , Some (r @ ExtremeExpr { which : Minimum , .. })) => (r , AlwaysFalse) , _ => return None , } } , Rel :: Le => { match (lx , rx) { (Some (l @ ExtremeExpr { which : Minimum , .. }) , _) => (l , AlwaysTrue) , (Some (l @ ExtremeExpr { which : Maximum , .. }) , _) => (l , InequalityImpossible) , (_ , Some (r @ ExtremeExpr { which : Minimum , .. })) => (r , InequalityImpossible) , (_ , Some (r @ ExtremeExpr { which : Maximum , .. })) => (r , AlwaysTrue) , _ => return None , } } , Rel :: Ne | Rel :: Eq => return None , }) }
};
}
