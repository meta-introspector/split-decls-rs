// Generated macro for impl_831 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_831 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_831"}
// Dependencies: {}
impl < St , F > FusedFuture for NextIf < '_ , St , F > where St : Stream , F : for < 'a > FnOnce1 < & 'a St :: Item , Output = bool > , { fn is_terminated (& self) -> bool { self . inner . is_none () } }
};
}
