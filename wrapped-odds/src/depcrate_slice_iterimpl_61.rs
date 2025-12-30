// Generated macro for impl_61 (impl)
macro_rules! Depcrate_slice_iterimpl_61 {
() => {
// Module: crate::slice::iter
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , T > Default for SliceCopyIter < 'a , T > where T : Copy , { # [doc = " Create an empty `SliceCopyIter`."] fn default () -> Self { unsafe { SliceCopyIter :: new (0x1 as * const T , 0x1 as * const T) } } }
};
}
