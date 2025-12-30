// Generated macro for impl_881 (impl)
macro_rules! Depcrate_stream_stream_takeimpl_881 {
() => {
// Module: crate::stream::stream::take
// Provides: {"impl_881"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Take < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
