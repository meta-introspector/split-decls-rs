// Generated macro for eq_range_end (function)
macro_rules! Depcrate_ast_utilseq_range_end {
() => {
// Module: crate::ast_utils
// Provides: {"eq_range_end"}
// Dependencies: {}
pub fn eq_range_end (l : & RangeEnd , r : & RangeEnd) -> bool { match (l , r) { (RangeEnd :: Excluded , RangeEnd :: Excluded) => true , (RangeEnd :: Included (l) , RangeEnd :: Included (r)) => { matches ! (l , RangeSyntax :: DotDotEq) == matches ! (r , RangeSyntax :: DotDotEq) } , _ => false , } }
};
}
