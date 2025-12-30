// Generated macro for impl_567 (impl)
macro_rules! Depcrate_stream_stream_unzipimpl_567 {
() => {
// Module: crate::stream::stream::unzip
// Provides: {"impl_567"}
// Dependencies: {}
impl < St , A , B , FromA , FromB > FusedFuture for Unzip < St , FromA , FromB > where St : FusedStream < Item = (A , B) > , FromA : Default + Extend < A > , FromB : Default + Extend < B > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
