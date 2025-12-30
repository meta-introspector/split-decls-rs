// Generated macro for impl_955 (impl)
macro_rules! Depcrate_stream_stream_try_foldimpl_955 {
() => {
// Module: crate::stream::stream::try_fold
// Provides: {"impl_955"}
// Dependencies: {}
impl < St , Fut , T , F > FusedFuture for TryFold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : TryFuture < Ok = T > , { fn is_terminated (& self) -> bool { self . accum . is_none () && self . future . is_none () } }
};
}
