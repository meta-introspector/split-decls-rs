// Generated macro for impl_801 (impl)
macro_rules! Depcrate_stream_stream_select_next_someimpl_801 {
() => {
// Module: crate::stream::stream::select_next_some
// Provides: {"impl_801"}
// Dependencies: {}
impl < St : ? Sized + FusedStream + Unpin > FusedFuture for SelectNextSome < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
