// Generated macro for impl_262 (impl)
macro_rules! Depcrate_options_errorimpl_262 {
() => {
// Module: crate::options::error
// Provides: {"impl_262"}
// Dependencies: {}
impl From < glob :: PatternError > for OptionsError { fn from (error : glob :: PatternError) -> Self { Self :: FailedGlobPattern (error . to_string ()) } }
};
}
