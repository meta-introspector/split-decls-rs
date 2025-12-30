// Generated macro for impl_709 (impl)
macro_rules! Depcrate_collectionimpl_709 {
() => {
// Module: crate::collection
// Provides: {"impl_709"}
// Dependencies: {}
# [doc = " Given `low ..= high`, then a size range `[low, high]` is the result."] impl From < RangeInclusive < usize > > for SizeRange { fn from (r : RangeInclusive < usize >) -> Self { size_range (* r . start () .. r . end () . saturating_add (1)) } }
};
}
