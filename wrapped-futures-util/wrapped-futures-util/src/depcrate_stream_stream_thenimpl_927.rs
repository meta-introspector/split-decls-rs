// Generated macro for impl_927 (impl)
macro_rules! Depcrate_stream_stream_thenimpl_927 {
() => {
// Module: crate::stream::stream::then
// Provides: {"impl_927"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for Then < St , Fut , F > where St : FusedStream , F : FnMut (St :: Item) -> Fut , Fut : Future , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
};
}
