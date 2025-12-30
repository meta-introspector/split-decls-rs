// Generated macro for is_none_arm_body (function)
macro_rules! Depcrate_matches_manual_utilsis_none_arm_body {
() => {
// Module: crate::matches::manual_utils
// Provides: {"is_none_arm_body"}
// Dependencies: {}
# [doc = " Checks for the `None` value, possibly in a block."] fn is_none_arm_body (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { is_none_expr (cx , peel_blocks (expr)) }
};
}
