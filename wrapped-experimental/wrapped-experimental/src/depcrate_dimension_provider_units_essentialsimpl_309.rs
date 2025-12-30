// Generated macro for impl_309 (impl)
macro_rules! Depcrate_dimension_provider_units_essentialsimpl_309 {
() => {
// Module: crate::dimension::provider::units::essentials
// Provides: {"impl_309"}
// Dependencies: {}
impl From < u8 > for CompoundCount { fn from (val : u8) -> Self { match val { 0 => CompoundCount :: Zero , 1 => CompoundCount :: One , 2 => CompoundCount :: Two , 3 => CompoundCount :: Few , 4 => CompoundCount :: Many , 5 => CompoundCount :: Other , _ => unreachable ! () , } } }
};
}
