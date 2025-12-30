// Generated macro for impl_184 (impl)
macro_rules! Depcrate_distr_uniformimpl_184 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_184"}
// Dependencies: {}
impl < X : SampleUniform > Distribution < X > for Uniform < X > { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> X { self . 0 . sample (rng) } }
};
}
