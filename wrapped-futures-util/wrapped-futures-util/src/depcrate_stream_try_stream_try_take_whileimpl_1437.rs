// Generated macro for impl_1437 (impl)
macro_rules! Depcrate_stream_try_stream_try_take_whileimpl_1437 {
() => {
// Module: crate::stream::try_stream::try_take_while
// Provides: {"impl_1437"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for TryTakeWhile < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { fn is_terminated (& self) -> bool { self . done_taking || self . pending_item . is_none () && self . stream . is_terminated () } }
};
}
