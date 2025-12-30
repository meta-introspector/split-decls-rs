// Generated macro for impl_580 (impl)
macro_rules! Depcrate_stream_stream_concatimpl_580 {
() => {
// Module: crate::stream::stream::concat
// Provides: {"impl_580"}
// Dependencies: {}
impl < St > FusedFuture for Concat < St > where St : FusedStream , St :: Item : Extend < < St :: Item as IntoIterator > :: Item > + IntoIterator + Default , { fn is_terminated (& self) -> bool { self . accum . is_none () && self . stream . is_terminated () } }
};
}
