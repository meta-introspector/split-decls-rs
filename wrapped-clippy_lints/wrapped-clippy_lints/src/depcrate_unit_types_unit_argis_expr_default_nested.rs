// Generated macro for is_expr_default_nested (function)
macro_rules! Depcrate_unit_types_unit_argis_expr_default_nested {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"is_expr_default_nested"}
// Dependencies: {}
fn is_expr_default_nested < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { is_expr_default (cx , expr) || matches ! (expr . kind , ExprKind :: Block (block , _) if block . expr . is_some () && is_expr_default_nested (cx , block . expr . unwrap ())) }
};
}
