// Generated macro for impl_1257 (impl)
macro_rules! Depcrate_stream_try_stream_try_filterimpl_1257 {
() => {
// Module: crate::stream::try_stream::try_filter
// Provides: {"impl_1257"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item , E > Sink < Item > for TryFilter < S , Fut , F > where S : TryStream + Sink < Item , Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
};
}
