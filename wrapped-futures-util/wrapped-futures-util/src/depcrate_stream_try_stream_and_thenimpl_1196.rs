// Generated macro for impl_1196 (impl)
macro_rules! Depcrate_stream_try_stream_and_thenimpl_1196 {
() => {
// Module: crate::stream::try_stream::and_then
// Provides: {"impl_1196"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for AndThen < S , Fut , F > where S : Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
