// Generated macro for test (module)
macro_rules! Depcrate_errortest {
() => {
// Module: crate::error
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_source_error () { let reason = RenderErrorReason :: TemplateNotFound ("unnamed" . to_owned ()) ; let render_error = RenderError :: from (reason) ; let reason2 = render_error . source () . unwrap () ; assert ! (matches ! (reason2 . downcast_ref ::< RenderErrorReason > () . unwrap () , RenderErrorReason :: TemplateNotFound (_))) ; } }
};
}
