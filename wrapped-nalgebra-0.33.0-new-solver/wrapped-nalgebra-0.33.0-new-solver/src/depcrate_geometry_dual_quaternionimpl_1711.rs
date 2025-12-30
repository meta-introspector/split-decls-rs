// Generated macro for impl_1711 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1711 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1711"}
// Dependencies: {}
impl < T : SimdRealField > Normed for DualQuaternion < T > { type Norm = T :: SimdRealField ; # [inline] fn norm (& self) -> T :: SimdRealField { self . real . norm () } # [inline] fn norm_squared (& self) -> T :: SimdRealField { self . real . norm_squared () } # [inline] fn scale_mut (& mut self , n : Self :: Norm) { self . real . scale_mut (n . clone ()) ; self . dual . scale_mut (n) ; } # [inline] fn unscale_mut (& mut self , n : Self :: Norm) { self . real . unscale_mut (n . clone ()) ; self . dual . unscale_mut (n) ; } }
};
}
