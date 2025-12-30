// Generated macro for c_cmp_to_ordering (function)
macro_rules! Depcrate_utilc_cmp_to_ordering {
() => {
// Module: crate::util
// Provides: {"c_cmp_to_ordering"}
// Dependencies: {}
pub fn c_cmp_to_ordering (cmp : c_int) -> Ordering { match cmp { 0 => Ordering :: Equal , n if n < 0 => Ordering :: Less , _ => Ordering :: Greater , } }
};
}
