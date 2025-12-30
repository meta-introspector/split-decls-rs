// Generated macro for impl_1106 (impl)
macro_rules! Depcrate_stream_stream_for_each_concurrentimpl_1106 {
() => {
// Module: crate::stream::stream::for_each_concurrent
// Provides: {"impl_1106"}
// Dependencies: {}
impl < St , Fut , F > FusedFuture for ForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { fn is_terminated (& self) -> bool { self . stream . is_none () && self . futures . is_empty () } }
};
}
