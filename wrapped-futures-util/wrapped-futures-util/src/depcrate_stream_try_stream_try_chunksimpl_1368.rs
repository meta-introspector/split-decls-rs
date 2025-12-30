// Generated macro for impl_1368 (impl)
macro_rules! Depcrate_stream_try_stream_try_chunksimpl_1368 {
() => {
// Module: crate::stream::try_stream::try_chunks
// Provides: {"impl_1368"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for TryChunks < S > where S : TryStream + Sink < Item > , { type Error = < S as Sink < Item > > :: Error ; delegate_sink ! (stream , Item) ; }
};
}
