// Generated macro for impl_1269 (impl)
macro_rules! Depcrate_stream_try_stream_try_forwardimpl_1269 {
() => {
// Module: crate::stream::try_stream::try_forward
// Provides: {"impl_1269"}
// Dependencies: {}
impl < St , Si , Item , E > FusedFuture for TryForward < St , Si , Item > where Si : Sink < Item , Error = E > , St : TryStream < Ok = Item , Error = E > , { fn is_terminated (& self) -> bool { self . sink . is_none () } }
};
}
