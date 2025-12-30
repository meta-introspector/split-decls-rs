// Generated macro for impl_1534 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1534 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1534"}
// Dependencies: {}
impl < T : SimdRealField > Normed for Quaternion < T > { type Norm = T :: SimdRealField ; # [inline] fn norm (& self) -> T :: SimdRealField { self . coords . norm () } # [inline] fn norm_squared (& self) -> T :: SimdRealField { self . coords . norm_squared () } # [inline] fn scale_mut (& mut self , n : Self :: Norm) { self . coords . scale_mut (n) } # [inline] fn unscale_mut (& mut self , n : Self :: Norm) { self . coords . unscale_mut (n) } }
};
}
