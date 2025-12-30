// Generated macro for impl_233 (impl)
macro_rules! Depcrate_distr_uniformimpl_233 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_233"}
// Dependencies: {}
impl < X : SampleUniform > TryFrom < Range < X > > for Uniform < X > { type Error = Error ; fn try_from (r : Range < X >) -> Result < Uniform < X > , Error > { Uniform :: new (r . start , r . end) } }
};
}
