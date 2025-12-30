// Generated macro for impl_705 (impl)
macro_rules! Depcrate_collectionimpl_705 {
() => {
// Module: crate::collection
// Provides: {"impl_705"}
// Dependencies: {}
# [doc = " Given `(low: usize, high: usize)`,"] # [doc = " then a size range of `[low..high)` is the result."] impl From < (usize , usize) > for SizeRange { fn from ((low , high) : (usize , usize)) -> Self { size_range (low .. high) } }
};
}
