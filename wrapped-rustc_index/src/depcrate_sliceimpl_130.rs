// Generated macro for impl_130 (impl)
macro_rules! Depcrate_sliceimpl_130 {
() => {
// Module: crate::slice
// Provides: {"impl_130"}
// Dependencies: {}
impl < I : Idx , T > Default for & IndexSlice < I , T > { # [inline] fn default () -> Self { IndexSlice :: from_raw (Default :: default ()) } }
};
}
