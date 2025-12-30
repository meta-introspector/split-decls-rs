// Generated macro for impl_1600 (impl)
macro_rules! Depcrate_geometry_quaternion_coordinatesimpl_1600 {
() => {
// Module: crate::geometry::quaternion_coordinates
// Provides: {"impl_1600"}
// Dependencies: {}
impl < T : Scalar + SimdValue > Deref for Quaternion < T > { type Target = IJKW < T > ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { & * (self as * const Self as * const Self :: Target) } } }
};
}
