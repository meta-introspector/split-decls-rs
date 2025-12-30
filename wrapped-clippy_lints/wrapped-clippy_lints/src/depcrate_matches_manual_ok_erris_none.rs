// Generated macro for is_none (function)
macro_rules! Depcrate_matches_manual_ok_erris_none {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"is_none"}
// Dependencies: {}
# [doc = " Check if `expr` is `None`, possibly as a block"] fn is_none (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { is_none_expr (cx , peel_blocks (expr)) }
};
}
