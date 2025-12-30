// Generated macro for impl_1286 (impl)
macro_rules! Depcrate_stream_try_stream_try_filter_mapimpl_1286 {
() => {
// Module: crate::stream::try_stream::try_filter_map
// Provides: {"impl_1286"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for TryFilterMap < S , Fut , F > where S : Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
