// Generated macro for impl_797 (impl)
macro_rules! Depcrate_base_conversionimpl_797 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_797"}
// Dependencies: {}
impl < 'a , T : Scalar > From < DVectorViewMut < 'a , T > > for & 'a mut [T] { fn from (vec : DVectorViewMut < 'a , T >) -> & 'a mut [T] { vec . data . into_slice_mut () } }
};
}
