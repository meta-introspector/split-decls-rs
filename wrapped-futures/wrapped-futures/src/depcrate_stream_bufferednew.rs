// Generated macro for new (function)
macro_rules! Depcrate_stream_bufferednew {
() => {
// Module: crate::stream::buffered
// Provides: {"new"}
// Dependencies: {}
pub fn new < S > (s : S , amt : usize) -> Buffered < S > where S : Stream , S :: Item : IntoFuture < Error = < S as Stream > :: Error > , { Buffered { stream : super :: fuse :: new (s) , futures : (0 .. amt) . map (| _ | None) . collect () , } }
};
}
