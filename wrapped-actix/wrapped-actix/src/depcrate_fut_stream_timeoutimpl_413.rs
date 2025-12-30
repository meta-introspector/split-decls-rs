// Generated macro for impl_413 (impl)
macro_rules! Depcrate_fut_stream_timeoutimpl_413 {
() => {
// Module: crate::fut::stream::timeout
// Provides: {"impl_413"}
// Dependencies: {}
impl < S > Timeout < S > { pub (super) fn new (stream : S , timeout : Duration) -> Self { Self { stream , dur : timeout , reset_timeout : false , timeout : sleep (timeout) , } } }
};
}
