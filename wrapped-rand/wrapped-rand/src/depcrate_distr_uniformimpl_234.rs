// Generated macro for impl_234 (impl)
macro_rules! Depcrate_distr_uniformimpl_234 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_234"}
// Dependencies: {}
impl < X : SampleUniform > TryFrom < RangeInclusive < X > > for Uniform < X > { type Error = Error ; fn try_from (r : :: core :: ops :: RangeInclusive < X >) -> Result < Uniform < X > , Error > { Uniform :: new_inclusive (r . start () , r . end ()) } }
};
}
