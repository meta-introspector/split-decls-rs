// Generated macro for impl_1255 (impl)
macro_rules! Depcrate_stream_try_stream_try_filterimpl_1255 {
() => {
// Module: crate::stream::try_stream::try_filter
// Provides: {"impl_1255"}
// Dependencies: {}
impl < St , Fut , F > FusedStream for TryFilter < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (& St :: Ok) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . pending_fut . is_none () && self . stream . is_terminated () } }
};
}
