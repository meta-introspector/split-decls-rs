// Generated macro for impl_188 (impl)
macro_rules! Depcrate_distr_uniformimpl_188 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_188"}
// Dependencies: {}
impl < X : SampleUniform > TryFrom < RangeInclusive < X > > for Uniform < X > { type Error = Error ; fn try_from (r : :: core :: ops :: RangeInclusive < X >) -> Result < Uniform < X > , Error > { Uniform :: new_inclusive (r . start () , r . end ()) } }
};
}
