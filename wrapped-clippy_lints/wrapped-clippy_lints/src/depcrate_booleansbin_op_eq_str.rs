// Generated macro for bin_op_eq_str (function)
macro_rules! Depcrate_booleansbin_op_eq_str {
() => {
// Module: crate::booleans
// Provides: {"bin_op_eq_str"}
// Dependencies: {}
fn bin_op_eq_str (op : BinOpKind) -> Option < & 'static str > { match op { BinOpKind :: Eq => Some ("==") , BinOpKind :: Ne => Some ("!=") , _ => None , } }
};
}
