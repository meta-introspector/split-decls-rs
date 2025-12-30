// Generated macro for is_empty_block (function)
macro_rules! Depcrate_unit_types_unit_argis_empty_block {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"is_empty_block"}
// Dependencies: {}
fn is_empty_block (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Block (Block { stmts : [] , expr : None , .. } , _ ,)) }
};
}
