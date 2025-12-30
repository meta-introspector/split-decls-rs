// Generated macro for impl_100 (impl)
macro_rules! Depcrate_errorimpl_100 {
() => {
// Module: crate::error
// Provides: {"impl_100"}
// Dependencies: {}
# [cfg (feature = "dir_source")] impl From < WalkdirError > for TemplateError { fn from (e : WalkdirError) -> TemplateError { TemplateError :: of (TemplateErrorReason :: from (e)) } }
};
}
