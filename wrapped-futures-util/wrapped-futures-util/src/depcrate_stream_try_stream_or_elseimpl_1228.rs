// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_stream_try_stream_or_elseimpl_1228 {
() => {
// Module: crate::stream::try_stream::or_else
// Provides: {"impl_1228"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for OrElse < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (St :: Error) -> Fut , Fut : TryFuture < Ok = St :: Ok > , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
};
}
