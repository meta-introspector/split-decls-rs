// Generated macro for bin_op_to_fcmp_predicate (function)
macro_rules! Depcrate_basebin_op_to_fcmp_predicate {
() => {
// Module: crate::base
// Provides: {"bin_op_to_fcmp_predicate"}
// Dependencies: {}
pub (crate) fn bin_op_to_fcmp_predicate (op : BinOp) -> RealPredicate { match op { BinOp :: Eq => RealPredicate :: RealOEQ , BinOp :: Ne => RealPredicate :: RealUNE , BinOp :: Lt => RealPredicate :: RealOLT , BinOp :: Le => RealPredicate :: RealOLE , BinOp :: Gt => RealPredicate :: RealOGT , BinOp :: Ge => RealPredicate :: RealOGE , op => bug ! ("bin_op_to_fcmp_predicate: expected comparison operator, found {:?}" , op) , } }
};
}
