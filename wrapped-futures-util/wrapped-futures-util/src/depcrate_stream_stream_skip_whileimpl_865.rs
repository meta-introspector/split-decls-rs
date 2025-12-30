// Generated macro for impl_865 (impl)
macro_rules! Depcrate_stream_stream_skip_whileimpl_865 {
() => {
// Module: crate::stream::stream::skip_while
// Provides: {"impl_865"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for SkipWhile < St , Fut , F > where St : FusedStream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . pending_item . is_none () && self . stream . is_terminated () } }
};
}
