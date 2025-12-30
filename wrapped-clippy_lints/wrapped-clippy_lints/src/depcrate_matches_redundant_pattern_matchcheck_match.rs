// Generated macro for check_match (function)
macro_rules! Depcrate_matches_redundant_pattern_matchcheck_match {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"check_match"}
// Dependencies: {}
pub (super) fn check_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op : & Expr < '_ > , arms : & [Arm < '_ >]) { if let Ok (arms) = arms . try_into () && let Some ((good_method , maybe_guard)) = found_good_method (cx , arms) { let span = is_expn_of (expr . span , sym :: matches) . unwrap_or (expr . span . to (op . span)) ; let result_expr = match & op . kind { ExprKind :: AddrOf (_ , _ , borrowed) => borrowed , _ => op , } ; let mut app = Applicability :: MachineApplicable ; let receiver_sugg = Sugg :: hir_with_applicability (cx , result_expr , "_" , & mut app) . maybe_paren () ; let mut sugg = format ! ("{receiver_sugg}.{good_method}") ; if let Some (guard) = maybe_guard { let has_nested_let_chain = for_each_expr_without_closures (guard , | expr | { if matches ! (expr . kind , ExprKind :: Let (..)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () ; if has_nested_let_chain { return ; } let guard = Sugg :: hir (cx , guard , "..") ; let _ = write ! (sugg , " && {}" , guard . maybe_paren ()) ; } span_lint_and_sugg (cx , REDUNDANT_PATTERN_MATCHING , span , format ! ("redundant pattern matching, consider using `{good_method}`") , "try" , sugg , app ,) ; } }
};
}
