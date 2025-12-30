// Generated macro for impl_926 (impl)
macro_rules! Depcrate_stream_stream_thenimpl_926 {
() => {
// Module: crate::stream::stream::then
// Provides: {"impl_926"}
// Dependencies: {}
impl < St , Fut , F > Then < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , future : None , f } } delegate_access_inner ! (stream , St , ()) ; }
};
}
