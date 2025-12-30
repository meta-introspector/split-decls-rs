// Generated macro for SseStream (struct)
macro_rules! Depcrate_providers_streamingSseStream {
() => {
// Module: crate::providers::streaming
// Provides: {"SseStream"}
// Dependencies: {}
# [doc = " Stream adapter that converts a reqwest Response stream into SSE events"] # [pin_project] pub struct SseStream < S > { # [pin] inner : S , buffer : String , }
};
}
