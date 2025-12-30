// Generated macro for impl_79 (impl)
macro_rules! Depcrate_slice_revimpl_79 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T > From < Box < [T] > > for Box < RevSlice < T > > { fn from (slc : Box < [T] >) -> Self { unsafe { transmute (slc) } } }
};
}
