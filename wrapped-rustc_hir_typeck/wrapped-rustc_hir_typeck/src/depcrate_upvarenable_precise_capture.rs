// Generated macro for enable_precise_capture (function)
macro_rules! Depcrate_upvarenable_precise_capture {
() => {
// Module: crate::upvar
// Provides: {"enable_precise_capture"}
// Dependencies: {}
# [doc = " Precise capture is enabled if user is using Rust Edition 2021 or higher."] # [doc = " `span` is the span of the closure."] fn enable_precise_capture (span : Span) -> bool { span . at_least_rust_2021 () }
};
}
