// Generated macro for impl_1468 (impl)
macro_rules! Depcrate_stream_try_stream_try_bufferedimpl_1468 {
() => {
// Module: crate::stream::try_stream::try_buffered
// Provides: {"impl_1468"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item , E > Sink < Item > for TryBuffered < S > where S : TryStream + Sink < Item , Error = E > , S :: Ok : TryFuture < Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
};
}
