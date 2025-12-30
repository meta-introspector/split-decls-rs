// Generated macro for is_none_expr (function)
macro_rules! Depcrate_matches_manual_utilsis_none_expr {
() => {
// Module: crate::matches::manual_utils
// Provides: {"is_none_expr"}
// Dependencies: {}
fn is_none_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { is_res_lang_ctor (cx , path_res (cx , peel_blocks (expr)) , OptionNone) }
};
}
