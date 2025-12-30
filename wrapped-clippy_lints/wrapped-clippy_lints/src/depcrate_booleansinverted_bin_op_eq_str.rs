// Generated macro for inverted_bin_op_eq_str (function)
macro_rules! Depcrate_booleansinverted_bin_op_eq_str {
() => {
// Module: crate::booleans
// Provides: {"inverted_bin_op_eq_str"}
// Dependencies: {}
fn inverted_bin_op_eq_str (op : BinOpKind) -> Option < & 'static str > { match op { BinOpKind :: Eq => Some ("!=") , BinOpKind :: Ne => Some ("==") , _ => None , } }
};
}
