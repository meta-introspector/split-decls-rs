// Generated macro for impl_70 (impl)
macro_rules! Depcrate_baseimpl_70 {
() => {
// Module: crate::base
// Provides: {"impl_70"}
// Dependencies: {}
impl From < CFComparisonResult > for Ordering { fn from (val : CFComparisonResult) -> Self { match val { CFComparisonResult :: LessThan => Ordering :: Less , CFComparisonResult :: EqualTo => Ordering :: Equal , CFComparisonResult :: GreaterThan => Ordering :: Greater , } } }
};
}
