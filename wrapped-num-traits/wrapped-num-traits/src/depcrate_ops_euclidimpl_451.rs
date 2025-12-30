// Generated macro for impl_451 (impl)
macro_rules! Depcrate_ops_euclidimpl_451 {
() => {
// Module: crate::ops::euclid
// Provides: {"impl_451"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl Euclid for f32 { # [inline] fn div_euclid (& self , v : & f32) -> f32 { let q = < f32 as crate :: float :: FloatCore > :: trunc (self / v) ; if self % v < 0.0 { return if * v > 0.0 { q - 1.0 } else { q + 1.0 } ; } q } # [inline] fn rem_euclid (& self , v : & f32) -> f32 { let r = self % v ; if r < 0.0 { r + < f32 as crate :: float :: FloatCore > :: abs (* v) } else { r } } }
};
}
