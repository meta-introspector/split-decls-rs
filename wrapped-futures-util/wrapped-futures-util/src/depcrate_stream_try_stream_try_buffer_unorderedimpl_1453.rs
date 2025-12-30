// Generated macro for impl_1453 (impl)
macro_rules! Depcrate_stream_try_stream_try_buffer_unorderedimpl_1453 {
() => {
// Module: crate::stream::try_stream::try_buffer_unordered
// Provides: {"impl_1453"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item , E > Sink < Item > for TryBufferUnordered < S > where S : TryStream + Sink < Item , Error = E > , S :: Ok : TryFuture < Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
};
}
