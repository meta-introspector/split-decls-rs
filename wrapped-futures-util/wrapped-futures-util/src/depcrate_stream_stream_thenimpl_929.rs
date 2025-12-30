// Generated macro for impl_929 (impl)
macro_rules! Depcrate_stream_stream_thenimpl_929 {
() => {
// Module: crate::stream::stream::then
// Provides: {"impl_929"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for Then < S , Fut , F > where S : Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
