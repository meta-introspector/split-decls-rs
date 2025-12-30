// Generated macro for impl_1016 (impl)
macro_rules! Depcrate_stream_stream_scanimpl_1016 {
() => {
// Module: crate::stream::stream::scan
// Provides: {"impl_1016"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < St , S , Fut , F , Item > Sink < Item > for Scan < St , S , Fut , F > where St : Stream + Sink < Item > , { type Error = St :: Error ; delegate_sink ! (stream , Item) ; }
};
}
