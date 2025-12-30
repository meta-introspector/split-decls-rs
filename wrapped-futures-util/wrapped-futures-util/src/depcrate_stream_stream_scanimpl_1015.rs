// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_stream_stream_scanimpl_1015 {
() => {
// Module: crate::stream::stream::scan
// Provides: {"impl_1015"}
// Dependencies: {}
impl < B , St , S , Fut , F > FusedStream for Scan < St , S , Fut , F > where St : FusedStream , F : FnMut (S , St :: Item) -> Fut , Fut : Future < Output = Option < (S , B) > > , { fn is_terminated (& self) -> bool { self . is_done_taking () || ! self . state . is_future () && self . stream . is_terminated () } }
};
}
