// Generated macro for impl_867 (impl)
macro_rules! Depcrate_stream_stream_skip_whileimpl_867 {
() => {
// Module: crate::stream::stream::skip_while
// Provides: {"impl_867"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for SkipWhile < S , Fut , F > where S : Stream + Sink < Item > , F : FnMut (& S :: Item) -> Fut , Fut : Future < Output = bool > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
