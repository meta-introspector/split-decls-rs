// Generated macro for impl_714 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_714 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_714"}
// Dependencies: {}
impl From < & Pattern < '_ > > for reference :: Pattern { fn from (input : & Pattern < '_ >) -> Self { Self { items : input . items . to_vec () , time_granularity : input . metadata . time_granularity () , } } }
};
}
