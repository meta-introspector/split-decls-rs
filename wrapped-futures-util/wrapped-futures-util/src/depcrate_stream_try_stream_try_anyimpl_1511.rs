// Generated macro for impl_1511 (impl)
macro_rules! Depcrate_stream_try_stream_try_anyimpl_1511 {
() => {
// Module: crate::stream::try_stream::try_any
// Provides: {"impl_1511"}
// Dependencies: {}
impl < St , Fut , F > FusedFuture for TryAny < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
};
}
