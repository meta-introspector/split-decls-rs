// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_stream_stream_try_for_each_concurrentimpl_1147 {
() => {
// Module: crate::stream::stream::try_for_each_concurrent
// Provides: {"impl_1147"}
// Dependencies: {}
impl < St , Fut , F , E > FusedFuture for TryForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = Result < () , E > > , { fn is_terminated (& self) -> bool { self . stream . is_none () && self . futures . is_empty () } }
};
}
