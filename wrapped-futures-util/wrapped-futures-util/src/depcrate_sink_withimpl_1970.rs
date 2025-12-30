// Generated macro for impl_1970 (impl)
macro_rules! Depcrate_sink_withimpl_1970 {
() => {
// Module: crate::sink::with
// Provides: {"impl_1970"}
// Dependencies: {}
impl < S , Item , U , Fut , F > Stream for With < S , Item , U , Fut , F > where S : Stream + Sink < Item > , F : FnMut (U) -> Fut , Fut : Future , { type Item = S :: Item ; delegate_stream ! (sink) ; }
};
}
