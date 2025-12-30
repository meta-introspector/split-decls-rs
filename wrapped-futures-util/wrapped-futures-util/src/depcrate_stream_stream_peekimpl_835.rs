// Generated macro for impl_835 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_835 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_835"}
// Dependencies: {}
impl < St , T > FusedFuture for NextIfEq < '_ , St , T > where St : Stream , T : ? Sized , St :: Item : PartialEq < T > , { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
};
}
