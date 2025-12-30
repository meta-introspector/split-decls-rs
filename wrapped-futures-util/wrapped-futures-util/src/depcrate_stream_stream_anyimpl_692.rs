// Generated macro for impl_692 (impl)
macro_rules! Depcrate_stream_stream_anyimpl_692 {
() => {
// Module: crate::stream::stream::any
// Provides: {"impl_692"}
// Dependencies: {}
impl < St , Fut , F > FusedFuture for Any < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
};
}
