// Generated macro for impl_121 (impl)
macro_rules! Depcrate_comparison_resultimpl_121 {
() => {
// Module: crate::comparison_result
// Provides: {"impl_121"}
// Dependencies: {}
impl From < Ordering > for NSComparisonResult { # [inline] fn from (order : Ordering) -> Self { match order { Ordering :: Less => Self :: Ascending , Ordering :: Equal => Self :: Same , Ordering :: Greater => Self :: Descending , } } }
};
}
