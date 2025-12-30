// Generated macro for impl_1350 (impl)
macro_rules! Depcrate_stream_try_stream_try_concatimpl_1350 {
() => {
// Module: crate::stream::try_stream::try_concat
// Provides: {"impl_1350"}
// Dependencies: {}
impl < St > TryConcat < St > where St : TryStream , St :: Ok : Extend < < St :: Ok as IntoIterator > :: Item > + IntoIterator + Default , { pub (super) fn new (stream : St) -> Self { Self { stream , accum : None } } }
};
}
