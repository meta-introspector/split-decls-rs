// Generated macro for check_all_arms (function)
macro_rules! Depcrate_matches_needless_matchcheck_all_arms {
() => {
// Module: crate::matches::needless_match
// Provides: {"check_all_arms"}
// Dependencies: {}
fn check_all_arms (cx : & LateContext < '_ > , match_expr : & Expr < '_ > , arms : & [Arm < '_ >]) -> bool { for arm in arms { let arm_expr = peel_blocks_with_stmt (arm . body) ; if let Some (guard_expr) = & arm . guard && guard_expr . can_have_side_effects () { return false ; } if let PatKind :: Wild = arm . pat . kind { if ! eq_expr_value (cx , match_expr , arm_expr) { return false ; } } else if ! pat_same_as_expr (arm . pat , arm_expr) { return false ; } } true }
};
}
