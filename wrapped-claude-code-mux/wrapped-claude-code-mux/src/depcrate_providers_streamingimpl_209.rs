// Generated macro for impl_209 (impl)
macro_rules! Depcrate_providers_streamingimpl_209 {
() => {
// Module: crate::providers::streaming
// Provides: {"impl_209"}
// Dependencies: {}
impl < S > SseStream < S > { pub fn new (stream : S) -> Self { Self { inner : stream , buffer : String :: new () , } } }
};
}
