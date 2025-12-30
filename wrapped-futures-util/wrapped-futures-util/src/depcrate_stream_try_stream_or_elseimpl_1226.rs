// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_stream_try_stream_or_elseimpl_1226 {
() => {
// Module: crate::stream::try_stream::or_else
// Provides: {"impl_1226"}
// Dependencies: {}
impl < St , Fut , F > OrElse < St , Fut , F > where St : TryStream , F : FnMut (St :: Error) -> Fut , Fut : TryFuture < Ok = St :: Ok > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , future : None , f } } delegate_access_inner ! (stream , St , ()) ; }
};
}
