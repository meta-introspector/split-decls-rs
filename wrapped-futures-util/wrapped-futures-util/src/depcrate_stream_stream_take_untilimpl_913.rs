// Generated macro for impl_913 (impl)
macro_rules! Depcrate_stream_stream_take_untilimpl_913 {
() => {
// Module: crate::stream::stream::take_until
// Provides: {"impl_913"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , Item > Sink < Item > for TakeUntil < S , Fut > where S : Stream + Sink < Item > , Fut : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
