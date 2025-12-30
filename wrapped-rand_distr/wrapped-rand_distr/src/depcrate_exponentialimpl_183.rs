// Generated macro for impl_183 (impl)
macro_rules! Depcrate_exponentialimpl_183 {
() => {
// Module: crate::exponential
// Provides: {"impl_183"}
// Dependencies: {}
impl Distribution < f32 > for Exp1 { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> f32 { let x : f64 = self . sample (rng) ; x as f32 } }
};
}
