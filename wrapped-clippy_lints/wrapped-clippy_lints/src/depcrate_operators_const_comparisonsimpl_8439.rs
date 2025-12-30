// Generated macro for impl_8439 (impl)
macro_rules! Depcrate_operators_const_comparisonsimpl_8439 {
() => {
// Module: crate::operators::const_comparisons
// Provides: {"impl_8439"}
// Dependencies: {}
impl TryFrom < BinOpKind > for CmpOp { type Error = () ; fn try_from (bin_op : BinOpKind) -> Result < Self , Self :: Error > { match bin_op { BinOpKind :: Lt => Ok (CmpOp :: Lt) , BinOpKind :: Le => Ok (CmpOp :: Le) , BinOpKind :: Ge => Ok (CmpOp :: Ge) , BinOpKind :: Gt => Ok (CmpOp :: Gt) , _ => Err (()) , } } }
};
}
