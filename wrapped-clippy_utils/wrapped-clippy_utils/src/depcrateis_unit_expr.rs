// Generated macro for is_unit_expr (function)
macro_rules! Depcrateis_unit_expr {
() => {
// Module: crate
// Provides: {"is_unit_expr"}
// Dependencies: {}
# [doc = " Checks if `expr` is an empty block or an empty tuple."] pub fn is_unit_expr (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Block (Block { stmts : [] , expr : None , .. } , _) | ExprKind :: Tup ([])) }
};
}
