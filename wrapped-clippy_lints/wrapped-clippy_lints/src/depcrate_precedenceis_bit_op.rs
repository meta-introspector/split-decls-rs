// Generated macro for is_bit_op (function)
macro_rules! Depcrate_precedenceis_bit_op {
() => {
// Module: crate::precedence
// Provides: {"is_bit_op"}
// Dependencies: {}
# [must_use] fn is_bit_op (op : BinOpKind) -> bool { matches ! (op , BitXor | BitAnd | BitOr | Shl | Shr) }
};
}
