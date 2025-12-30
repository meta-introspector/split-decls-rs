// Generated macro for left_side_is_useless (function)
macro_rules! Depcrate_operators_const_comparisonsleft_side_is_useless {
() => {
// Module: crate::operators::const_comparisons
// Provides: {"left_side_is_useless"}
// Dependencies: {}
fn left_side_is_useless (left_cmp_op : CmpOp , ordering : Ordering) -> bool { if ordering == Ordering :: Equal { match left_cmp_op { CmpOp :: Lt | CmpOp :: Gt => false , CmpOp :: Le | CmpOp :: Ge => true , } } else { match (left_cmp_op . direction () , ordering) { (CmpOpDirection :: Lesser , Ordering :: Less) => false , (CmpOpDirection :: Lesser , Ordering :: Equal) => false , (CmpOpDirection :: Lesser , Ordering :: Greater) => true , (CmpOpDirection :: Greater , Ordering :: Less) => true , (CmpOpDirection :: Greater , Ordering :: Equal) => false , (CmpOpDirection :: Greater , Ordering :: Greater) => false , } } }
};
}
