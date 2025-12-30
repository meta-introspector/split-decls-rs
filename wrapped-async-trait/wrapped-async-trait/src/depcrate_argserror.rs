// Generated macro for error (function)
macro_rules! Depcrate_argserror {
() => {
// Module: crate::args
// Provides: {"error"}
// Dependencies: {}
fn error () -> Error { let msg = "expected #[async_trait] or #[async_trait(?Send)]" ; Error :: new (Span :: call_site () , msg) }
};
}
