// Generated macro for impl_452 (impl)
macro_rules! Depcrate_ops_euclidimpl_452 {
() => {
// Module: crate::ops::euclid
// Provides: {"impl_452"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl Euclid for f64 { # [inline] fn div_euclid (& self , v : & f64) -> f64 { let q = < f64 as crate :: float :: FloatCore > :: trunc (self / v) ; if self % v < 0.0 { return if * v > 0.0 { q - 1.0 } else { q + 1.0 } ; } q } # [inline] fn rem_euclid (& self , v : & f64) -> f64 { let r = self % v ; if r < 0.0 { r + < f64 as crate :: float :: FloatCore > :: abs (* v) } else { r } } }
};
}
