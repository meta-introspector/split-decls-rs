// Generated macro for impl_984 (impl)
macro_rules! Depcrate_stream_stream_chunksimpl_984 {
() => {
// Module: crate::stream::stream::chunks
// Provides: {"impl_984"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Chunks < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
