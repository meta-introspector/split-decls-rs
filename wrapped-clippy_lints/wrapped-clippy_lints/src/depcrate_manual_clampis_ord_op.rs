// Generated macro for is_ord_op (function)
macro_rules! Depcrate_manual_clampis_ord_op {
() => {
// Module: crate::manual_clamp
// Provides: {"is_ord_op"}
// Dependencies: {}
fn is_ord_op (op : BinOpKind) -> bool { matches ! (op , BinOpKind :: Ge | BinOpKind :: Gt | BinOpKind :: Le | BinOpKind :: Lt) }
};
}
