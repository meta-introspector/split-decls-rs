// Generated macro for impl_1421 (impl)
macro_rules! Depcrate_stream_try_stream_try_skip_whileimpl_1421 {
() => {
// Module: crate::stream::try_stream::try_skip_while
// Provides: {"impl_1421"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for TrySkipWhile < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { fn is_terminated (& self) -> bool { self . pending_item . is_none () && self . stream . is_terminated () } }
};
}
