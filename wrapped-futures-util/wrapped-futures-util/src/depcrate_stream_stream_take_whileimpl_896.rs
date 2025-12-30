// Generated macro for impl_896 (impl)
macro_rules! Depcrate_stream_stream_take_whileimpl_896 {
() => {
// Module: crate::stream::stream::take_while
// Provides: {"impl_896"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for TakeWhile < St , Fut , F > where St : FusedStream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done_taking || self . pending_item . is_none () && self . stream . is_terminated () } }
};
}
