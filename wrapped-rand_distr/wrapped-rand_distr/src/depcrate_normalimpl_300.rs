// Generated macro for impl_300 (impl)
macro_rules! Depcrate_normalimpl_300 {
() => {
// Module: crate::normal
// Provides: {"impl_300"}
// Dependencies: {}
impl Distribution < f32 > for StandardNormal { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> f32 { let x : f64 = self . sample (rng) ; x as f32 } }
};
}
