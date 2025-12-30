// Generated macro for impl_652 (impl)
macro_rules! Depcrate_stream_stream_filter_mapimpl_652 {
() => {
// Module: crate::stream::stream::filter_map
// Provides: {"impl_652"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for FilterMap < S , Fut , F > where S : Stream + Sink < Item > , F : FnMut1 < S :: Item , Output = Fut > , Fut : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
