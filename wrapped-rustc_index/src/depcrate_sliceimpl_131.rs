// Generated macro for impl_131 (impl)
macro_rules! Depcrate_sliceimpl_131 {
() => {
// Module: crate::slice
// Provides: {"impl_131"}
// Dependencies: {}
impl < I : Idx , T > Default for & mut IndexSlice < I , T > { # [inline] fn default () -> Self { IndexSlice :: from_raw_mut (Default :: default ()) } }
};
}
