// Generated macro for impl_794 (impl)
macro_rules! Depcrate_base_conversionimpl_794 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_794"}
// Dependencies: {}
impl < 'a , T : Scalar + Copy > From < & 'a [T] > for DVectorView < 'a , T > { # [inline] fn from (slice : & 'a [T]) -> Self { Self :: from_slice (slice , slice . len ()) } }
};
}
