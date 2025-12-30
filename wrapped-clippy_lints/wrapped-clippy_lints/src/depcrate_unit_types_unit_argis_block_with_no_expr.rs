// Generated macro for is_block_with_no_expr (function)
macro_rules! Depcrate_unit_types_unit_argis_block_with_no_expr {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"is_block_with_no_expr"}
// Dependencies: {}
fn is_block_with_no_expr (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Block (Block { expr : None , .. } , _)) }
};
}
