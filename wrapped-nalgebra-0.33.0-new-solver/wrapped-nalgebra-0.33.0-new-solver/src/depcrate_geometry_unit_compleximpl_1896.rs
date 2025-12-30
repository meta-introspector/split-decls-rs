// Generated macro for impl_1896 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1896 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1896"}
// Dependencies: {}
impl < T : SimdRealField > Normed for Complex < T > { type Norm = T :: SimdRealField ; # [inline] fn norm (& self) -> T :: SimdRealField { (self . re . clone () * self . re . clone () + self . im . clone () * self . im . clone ()) . simd_sqrt () } # [inline] fn norm_squared (& self) -> T :: SimdRealField { self . re . clone () * self . re . clone () + self . im . clone () * self . im . clone () } # [inline] fn scale_mut (& mut self , n : Self :: Norm) { self . re *= n . clone () ; self . im *= n ; } # [inline] fn unscale_mut (& mut self , n : Self :: Norm) { self . re /= n . clone () ; self . im /= n ; } }
};
}
