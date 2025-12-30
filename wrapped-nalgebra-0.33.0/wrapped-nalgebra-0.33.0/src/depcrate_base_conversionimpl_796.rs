// Generated macro for impl_796 (impl)
macro_rules! Depcrate_base_conversionimpl_796 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_796"}
// Dependencies: {}
impl < 'a , T : Scalar + Copy > From < & 'a mut [T] > for DVectorViewMut < 'a , T > { # [inline] fn from (slice : & 'a mut [T]) -> Self { Self :: from_slice (slice , slice . len ()) } }
};
}
