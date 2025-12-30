// Generated macro for impl_187 (impl)
macro_rules! Depcrate_distr_uniformimpl_187 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_187"}
// Dependencies: {}
impl < X : SampleUniform > TryFrom < Range < X > > for Uniform < X > { type Error = Error ; fn try_from (r : Range < X >) -> Result < Uniform < X > , Error > { Uniform :: new (r . start , r . end) } }
};
}
