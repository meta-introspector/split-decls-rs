// Generated macro for check (function)
macro_rules! Depcrate_operators_absurd_extreme_comparisonscheck {
() => {
// Module: crate::operators::absurd_extreme_comparisons
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op : BinOpKind , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > ,) { if let Some ((culprit , result)) = detect_absurd_comparison (cx , op , lhs , rhs) { let msg = "this comparison involving the minimum or maximum element for this \
                           type contains a case that is always true or always false" ; let conclusion = match result { AbsurdComparisonResult :: AlwaysFalse => "this comparison is always false" . to_owned () , AbsurdComparisonResult :: AlwaysTrue => "this comparison is always true" . to_owned () , AbsurdComparisonResult :: InequalityImpossible => format ! ("the case where the two sides are not equal never occurs, consider using `{} == {}` \
                         instead" , snippet (cx , lhs . span , "lhs") , snippet (cx , rhs . span , "rhs")) , } ; let help = format ! ("because `{}` is the {} value for this type, {conclusion}" , snippet (cx , culprit . expr . span , "x") , match culprit . which { ExtremeType :: Minimum => "minimum" , ExtremeType :: Maximum => "maximum" , }) ; span_lint_and_help (cx , ABSURD_EXTREME_COMPARISONS , expr . span , msg , None , help) ; } }
};
}
