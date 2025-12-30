// Generated macro for impl_578 (impl)
macro_rules! Depcrate_stream_stream_concatimpl_578 {
() => {
// Module: crate::stream::stream::concat
// Provides: {"impl_578"}
// Dependencies: {}
impl < St > Concat < St > where St : Stream , St :: Item : Extend < < St :: Item as IntoIterator > :: Item > + IntoIterator + Default , { pub (super) fn new (stream : St) -> Self { Self { stream , accum : None } } }
};
}
