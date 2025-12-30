// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_stream_stream_bufferedimpl_1051 {
() => {
// Module: crate::stream::stream::buffered
// Provides: {"impl_1051"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Buffered < S > where S : Stream + Sink < Item > , S :: Item : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
