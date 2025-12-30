// Generated macro for impl_1388 (impl)
macro_rules! Depcrate_stream_try_stream_try_ready_chunksimpl_1388 {
() => {
// Module: crate::stream::try_stream::try_ready_chunks
// Provides: {"impl_1388"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for TryReadyChunks < S > where S : TryStream + Sink < Item > , { type Error = < S as Sink < Item > > :: Error ; delegate_sink ! (stream , Item) ; }
};
}
