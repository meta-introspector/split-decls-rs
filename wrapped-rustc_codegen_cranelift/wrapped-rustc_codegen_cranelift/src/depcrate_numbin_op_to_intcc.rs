// Generated macro for bin_op_to_intcc (function)
macro_rules! Depcrate_numbin_op_to_intcc {
() => {
// Module: crate::num
// Provides: {"bin_op_to_intcc"}
// Dependencies: {}
fn bin_op_to_intcc (bin_op : BinOp , signed : bool) -> IntCC { use BinOp :: * ; use IntCC :: * ; match bin_op { Eq => Equal , Lt => { if signed { SignedLessThan } else { UnsignedLessThan } } Le => { if signed { SignedLessThanOrEqual } else { UnsignedLessThanOrEqual } } Ne => NotEqual , Ge => { if signed { SignedGreaterThanOrEqual } else { UnsignedGreaterThanOrEqual } } Gt => { if signed { SignedGreaterThan } else { UnsignedGreaterThan } } _ => unreachable ! () , } }
};
}
