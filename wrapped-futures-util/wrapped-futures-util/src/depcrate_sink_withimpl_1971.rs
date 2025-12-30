// Generated macro for impl_1971 (impl)
macro_rules! Depcrate_sink_withimpl_1971 {
() => {
// Module: crate::sink::with
// Provides: {"impl_1971"}
// Dependencies: {}
impl < S , Item , U , Fut , F > FusedStream for With < S , Item , U , Fut , F > where S : FusedStream + Sink < Item > , F : FnMut (U) -> Fut , Fut : Future , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
};
}
