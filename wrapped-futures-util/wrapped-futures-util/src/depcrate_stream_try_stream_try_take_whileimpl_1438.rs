// Generated macro for impl_1438 (impl)
macro_rules! Depcrate_stream_try_stream_try_take_whileimpl_1438 {
() => {
// Module: crate::stream::try_stream::try_take_while
// Provides: {"impl_1438"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item , E > Sink < Item > for TryTakeWhile < S , Fut , F > where S : TryStream + Sink < Item , Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
};
}
