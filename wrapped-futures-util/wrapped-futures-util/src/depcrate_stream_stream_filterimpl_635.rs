// Generated macro for impl_635 (impl)
macro_rules! Depcrate_stream_stream_filterimpl_635 {
() => {
// Module: crate::stream::stream::filter
// Provides: {"impl_635"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for Filter < S , Fut , F > where S : Stream + Sink < Item > , F : FnMut (& S :: Item) -> Fut , Fut : Future < Output = bool > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
