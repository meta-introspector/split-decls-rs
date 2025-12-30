// Generated macro for check (function)
macro_rules! Depcrate_methods_clear_with_draincheck {
() => {
// Module: crate::methods::clear_with_drain
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , span : Span , arg : Option < & Expr < '_ > >) { if let Some (arg) = arg { if match_acceptable_type (cx , recv , & ACCEPTABLE_TYPES_WITH_ARG) && let ExprKind :: Path (QPath :: Resolved (None , container_path)) = recv . kind && is_range_full (cx , arg , Some (container_path)) { suggest (cx , expr , recv , span) ; } } else if match_acceptable_type (cx , recv , & ACCEPTABLE_TYPES_WITHOUT_ARG) { suggest (cx , expr , recv , span) ; } }
};
}
