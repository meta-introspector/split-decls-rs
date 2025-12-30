// Generated macro for impl_720 (impl)
macro_rules! Depcrate_stream_stream_forwardimpl_720 {
() => {
// Module: crate::stream::stream::forward
// Provides: {"impl_720"}
// Dependencies: {}
impl < St , Si , Item , E > FusedFuture for Forward < St , Si , Item > where Si : Sink < Item , Error = E > , St : Stream < Item = Item > , { fn is_terminated (& self) -> bool { self . sink . is_none () } }
};
}
