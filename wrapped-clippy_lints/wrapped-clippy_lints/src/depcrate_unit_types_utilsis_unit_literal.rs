// Generated macro for is_unit_literal (function)
macro_rules! Depcrate_unit_types_utilsis_unit_literal {
() => {
// Module: crate::unit_types::utils
// Provides: {"is_unit_literal"}
// Dependencies: {}
pub (super) fn is_unit_literal (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Tup (slice) if slice . is_empty ()) }
};
}
