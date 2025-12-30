// Generated macro for impl_710 (impl)
macro_rules! Depcrate_collectionimpl_710 {
() => {
// Module: crate::collection
// Provides: {"impl_710"}
// Dependencies: {}
# [doc = " Given `..=high`, then a size range `[0, high]` is the result."] impl From < RangeToInclusive < usize > > for SizeRange { fn from (high : RangeToInclusive < usize >) -> Self { size_range (0 ..= high . end) } }
};
}
