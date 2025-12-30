// Generated macro for is_branchy (function)
macro_rules! Depcrate_undocumented_unsafe_blocksis_branchy {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"is_branchy"}
// Dependencies: {}
# [doc = " Checks if an expression is \"branchy\", e.g. loop, match/if/etc."] fn is_branchy (expr : & hir :: Expr < '_ >) -> bool { matches ! (expr . kind , hir :: ExprKind :: If (..) | hir :: ExprKind :: Loop (..) | hir :: ExprKind :: Match (..)) }
};
}
