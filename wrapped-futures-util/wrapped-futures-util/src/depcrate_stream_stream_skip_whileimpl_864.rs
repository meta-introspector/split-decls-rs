// Generated macro for impl_864 (impl)
macro_rules! Depcrate_stream_stream_skip_whileimpl_864 {
() => {
// Module: crate::stream::stream::skip_while
// Provides: {"impl_864"}
// Dependencies: {}
impl < St , Fut , F > SkipWhile < St , Fut , F > where St : Stream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_skipping : false } } delegate_access_inner ! (stream , St , ()) ; }
};
}
