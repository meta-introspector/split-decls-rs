// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < S , F > TracingService < S , F > { # [doc = " Constructs new tracing middleware."] pub fn new (inner : S , make_span : F) -> Self { TracingService { inner , make_span } } }
};
}
