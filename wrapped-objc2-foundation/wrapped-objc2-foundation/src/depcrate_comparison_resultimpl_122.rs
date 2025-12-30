// Generated macro for impl_122 (impl)
macro_rules! Depcrate_comparison_resultimpl_122 {
() => {
// Module: crate::comparison_result
// Provides: {"impl_122"}
// Dependencies: {}
impl From < NSComparisonResult > for Ordering { # [inline] fn from (comparison_result : NSComparisonResult) -> Self { match comparison_result { NSComparisonResult :: Ascending => Self :: Less , NSComparisonResult :: Same => Self :: Equal , NSComparisonResult :: Descending => Self :: Greater , } } }
};
}
