// Generated macro for impl_239 (impl)
macro_rules! Depcrate_distr_uniformimpl_239 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_239"}
// Dependencies: {}
impl < T : SampleUniform + PartialOrd > SampleRange < T > for Range < T > { # [inline] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result < T , Error > { T :: Sampler :: sample_single (self . start , self . end , rng) } # [inline] fn is_empty (& self) -> bool { ! (self . start < self . end) } }
};
}
