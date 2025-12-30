// Generated macro for impl_1422 (impl)
macro_rules! Depcrate_stream_try_stream_try_skip_whileimpl_1422 {
() => {
// Module: crate::stream::try_stream::try_skip_while
// Provides: {"impl_1422"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item , E > Sink < Item > for TrySkipWhile < S , Fut , F > where S : TryStream + Sink < Item , Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
};
}
