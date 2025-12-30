// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl From < FeatureGateError > for ProgramError { fn from (e : FeatureGateError) -> Self { ProgramError :: Custom (e as u32) } }
};
}
