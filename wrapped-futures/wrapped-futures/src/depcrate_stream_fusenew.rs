// Generated macro for new (function)
macro_rules! Depcrate_stream_fusenew {
() => {
// Module: crate::stream::fuse
// Provides: {"new"}
// Dependencies: {}
pub fn new < S : Stream > (s : S) -> Fuse < S > { Fuse { stream : Some (s) } }
};
}
