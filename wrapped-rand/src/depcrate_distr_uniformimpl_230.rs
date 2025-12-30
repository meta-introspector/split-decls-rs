// Generated macro for impl_230 (impl)
macro_rules! Depcrate_distr_uniformimpl_230 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_230"}
// Dependencies: {}
impl < X : SampleUniform > Distribution < X > for Uniform < X > { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> X { self . 0 . sample (rng) } }
};
}
