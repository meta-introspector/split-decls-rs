// Generated macro for impl_897 (impl)
macro_rules! Depcrate_stream_stream_take_whileimpl_897 {
() => {
// Module: crate::stream::stream::take_while
// Provides: {"impl_897"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for TakeWhile < S , Fut , F > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
