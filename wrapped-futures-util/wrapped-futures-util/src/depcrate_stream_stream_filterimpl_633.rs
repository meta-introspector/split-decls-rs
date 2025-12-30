// Generated macro for impl_633 (impl)
macro_rules! Depcrate_stream_stream_filterimpl_633 {
() => {
// Module: crate::stream::stream::filter
// Provides: {"impl_633"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for Filter < St , Fut , F > where St : Stream + FusedStream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . pending_fut . is_none () && self . stream . is_terminated () } }
};
}
