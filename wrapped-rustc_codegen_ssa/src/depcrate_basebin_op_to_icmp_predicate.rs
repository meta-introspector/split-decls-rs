// Generated macro for bin_op_to_icmp_predicate (function)
macro_rules! Depcrate_basebin_op_to_icmp_predicate {
() => {
// Module: crate::base
// Provides: {"bin_op_to_icmp_predicate"}
// Dependencies: {}
pub (crate) fn bin_op_to_icmp_predicate (op : BinOp , signed : bool) -> IntPredicate { match (op , signed) { (BinOp :: Eq , _) => IntPredicate :: IntEQ , (BinOp :: Ne , _) => IntPredicate :: IntNE , (BinOp :: Lt , true) => IntPredicate :: IntSLT , (BinOp :: Lt , false) => IntPredicate :: IntULT , (BinOp :: Le , true) => IntPredicate :: IntSLE , (BinOp :: Le , false) => IntPredicate :: IntULE , (BinOp :: Gt , true) => IntPredicate :: IntSGT , (BinOp :: Gt , false) => IntPredicate :: IntUGT , (BinOp :: Ge , true) => IntPredicate :: IntSGE , (BinOp :: Ge , false) => IntPredicate :: IntUGE , op => bug ! ("bin_op_to_icmp_predicate: expected comparison operator, found {:?}" , op) , } }
};
}
