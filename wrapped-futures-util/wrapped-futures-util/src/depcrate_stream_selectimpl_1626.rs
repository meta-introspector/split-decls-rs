// Generated macro for impl_1626 (impl)
macro_rules! Depcrate_stream_selectimpl_1626 {
() => {
// Module: crate::stream::select
// Provides: {"impl_1626"}
// Dependencies: {}
impl < St1 , St2 > FusedStream for Select < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
};
}
