// Generated macro for impl_678 (impl)
macro_rules! Depcrate_stream_stream_foldimpl_678 {
() => {
// Module: crate::stream::stream::fold
// Provides: {"impl_678"}
// Dependencies: {}
impl < St , Fut , T , F > FusedFuture for Fold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : Future < Output = T > , { fn is_terminated (& self) -> bool { self . accum . is_none () && self . future . is_none () } }
};
}
