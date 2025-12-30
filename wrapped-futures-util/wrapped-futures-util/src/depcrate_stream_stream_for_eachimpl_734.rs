// Generated macro for impl_734 (impl)
macro_rules! Depcrate_stream_stream_for_eachimpl_734 {
() => {
// Module: crate::stream::stream::for_each
// Provides: {"impl_734"}
// Dependencies: {}
impl < St , Fut , F > FusedFuture for ForEach < St , Fut , F > where St : FusedStream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
};
}
