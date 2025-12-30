// Generated macro for impl_1601 (impl)
macro_rules! Depcrate_geometry_quaternion_coordinatesimpl_1601 {
() => {
// Module: crate::geometry::quaternion_coordinates
// Provides: {"impl_1601"}
// Dependencies: {}
impl < T : Scalar + SimdValue > DerefMut for Quaternion < T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * (self as * mut Self as * mut Self :: Target) } } }
};
}
