// Generated macro for comparison_is_possible (function)
macro_rules! Depcrate_operators_const_comparisonscomparison_is_possible {
() => {
// Module: crate::operators::const_comparisons
// Provides: {"comparison_is_possible"}
// Dependencies: {}
fn comparison_is_possible (left_cmp_direction : CmpOpDirection , ordering : Ordering) -> bool { match (left_cmp_direction , ordering) { (CmpOpDirection :: Lesser , Ordering :: Less | Ordering :: Equal) => false , (CmpOpDirection :: Lesser , Ordering :: Greater) => true , (CmpOpDirection :: Greater , Ordering :: Greater | Ordering :: Equal) => false , (CmpOpDirection :: Greater , Ordering :: Less) => true , } }
};
}
