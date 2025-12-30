// Generated macro for impl_194 (impl)
macro_rules! Depcrate_distr_uniformimpl_194 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_194"}
// Dependencies: {}
impl < T : SampleUniform + PartialOrd > SampleRange < T > for RangeInclusive < T > { # [inline] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result < T , Error > { T :: Sampler :: sample_single_inclusive (self . start () , self . end () , rng) } # [inline] fn is_empty (& self) -> bool { ! (self . start () <= self . end ()) } }
};
}
