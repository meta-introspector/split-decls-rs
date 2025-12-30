// Generated macro for path_to_local (function)
macro_rules! Depcratepath_to_local {
() => {
// Module: crate
// Provides: {"path_to_local"}
// Dependencies: {}
# [doc = " If the expression is a path to a local, returns the canonical `HirId` of the local."] pub fn path_to_local (expr : & Expr < '_ >) -> Option < HirId > { if let ExprKind :: Path (QPath :: Resolved (None , path)) = expr . kind && let Res :: Local (id) = path . res { return Some (id) ; } None }
};
}
