// Generated macro for check_for_parens (function)
macro_rules! Depcrate_needless_parens_on_range_literalscheck_for_parens {
() => {
// Module: crate::needless_parens_on_range_literals
// Provides: {"check_for_parens"}
// Dependencies: {}
fn check_for_parens (cx : & LateContext < '_ > , e : & Expr < '_ > , is_start : bool) { if is_start && let ExprKind :: Lit (literal) = e . kind && let ast :: LitKind :: Float (_sym , ast :: LitFloatType :: Unsuffixed) = literal . node { return ; } if let ExprKind :: Lit (literal) = e . kind && (literal . span . data () . hi - literal . span . data () . lo) != (e . span . data () . hi - e . span . data () . lo) && snippet_enclosed_in_parenthesis (& snippet (cx , e . span , "")) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_then (cx , NEEDLESS_PARENS_ON_RANGE_LITERALS , e . span , "needless parenthesis on range literals can be removed" , | diag | { let suggestion = snippet_with_applicability (cx , literal . span , "_" , & mut applicability) ; diag . span_suggestion (e . span , "try" , suggestion , applicability) ; } ,) ; } }
};
}
