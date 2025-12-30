// Generated macro for impl_70 (impl)
macro_rules! Depcrate_baseimpl_70 {
() => {
// Module: crate::base
// Provides: {"impl_70"}
// Dependencies: {}
impl From < Ordering > for CFComparisonResult { # [inline] fn from (order : Ordering) -> Self { match order { Ordering :: Less => Self :: CompareLessThan , Ordering :: Equal => Self :: CompareEqualTo , Ordering :: Greater => Self :: CompareGreaterThan , } } }
};
}
