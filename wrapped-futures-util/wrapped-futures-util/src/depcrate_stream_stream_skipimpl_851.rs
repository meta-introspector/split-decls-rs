// Generated macro for impl_851 (impl)
macro_rules! Depcrate_stream_stream_skipimpl_851 {
() => {
// Module: crate::stream::stream::skip
// Provides: {"impl_851"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Skip < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
