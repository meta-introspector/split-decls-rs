// Generated macro for impl_998 (impl)
macro_rules! Depcrate_stream_stream_ready_chunksimpl_998 {
() => {
// Module: crate::stream::stream::ready_chunks
// Provides: {"impl_998"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for ReadyChunks < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
