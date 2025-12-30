// Generated macro for impl_206 (impl)
macro_rules! Depcrate_providers_streamingimpl_206 {
() => {
// Module: crate::providers::streaming
// Provides: {"impl_206"}
// Dependencies: {}
impl SseEvent { # [doc = " Format as SSE output for client"] pub fn to_sse_string (& self) -> String { let mut output = String :: new () ; if let Some (ref event_type) = self . event { output . push_str (& format ! ("event: {}\n" , event_type)) ; } output . push_str (& format ! ("data: {}\n\n" , self . data)) ; output } }
};
}
