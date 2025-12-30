// Generated macro for new (function)
macro_rules! Depcrate_stream_futurenew {
() => {
// Module: crate::stream::future
// Provides: {"new"}
// Dependencies: {}
pub fn new < S : Stream > (s : S) -> StreamFuture < S > { StreamFuture { stream : Some (s) } }
};
}
