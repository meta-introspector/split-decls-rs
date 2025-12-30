// Generated macro for impl_1497 (impl)
macro_rules! Depcrate_stream_try_stream_try_allimpl_1497 {
() => {
// Module: crate::stream::try_stream::try_all
// Provides: {"impl_1497"}
// Dependencies: {}
impl < St , Fut , F > FusedFuture for TryAll < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
};
}
