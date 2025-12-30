// Generated macro for impl_706 (impl)
macro_rules! Depcrate_stream_stream_allimpl_706 {
() => {
// Module: crate::stream::stream::all
// Provides: {"impl_706"}
// Dependencies: {}
impl < St , Fut , F > FusedFuture for All < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
};
}
