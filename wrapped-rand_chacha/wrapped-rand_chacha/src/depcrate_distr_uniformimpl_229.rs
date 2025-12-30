// Generated macro for impl_229 (impl)
macro_rules! Depcrate_distr_uniformimpl_229 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_229"}
// Dependencies: {}
impl < X : SampleUniform > Uniform < X > { # [doc = " Create a new `Uniform` instance, which samples uniformly from the half"] # [doc = " open range `[low, high)` (excluding `high`)."] # [doc = ""] # [doc = " For discrete types (e.g. integers), samples will always be strictly less"] # [doc = " than `high`. For (approximations of) continuous types (e.g. `f32`, `f64`),"] # [doc = " samples may equal `high` due to loss of precision but may not be"] # [doc = " greater than `high`."] # [doc = ""] # [doc = " Fails if `low >= high`, or if `low`, `high` or the range `high - low` is"] # [doc = " non-finite. In release mode, only the range is checked."] pub fn new < B1 , B2 > (low : B1 , high : B2) -> Result < Uniform < X > , Error > where B1 : SampleBorrow < X > + Sized , B2 : SampleBorrow < X > + Sized , { X :: Sampler :: new (low , high) . map (Uniform) } # [doc = " Create a new `Uniform` instance, which samples uniformly from the closed"] # [doc = " range `[low, high]` (inclusive)."] # [doc = ""] # [doc = " Fails if `low > high`, or if `low`, `high` or the range `high - low` is"] # [doc = " non-finite. In release mode, only the range is checked."] pub fn new_inclusive < B1 , B2 > (low : B1 , high : B2) -> Result < Uniform < X > , Error > where B1 : SampleBorrow < X > + Sized , B2 : SampleBorrow < X > + Sized , { X :: Sampler :: new_inclusive (low , high) . map (Uniform) } }
};
}
