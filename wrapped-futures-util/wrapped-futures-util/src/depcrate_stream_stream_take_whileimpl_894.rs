// Generated macro for impl_894 (impl)
macro_rules! Depcrate_stream_stream_take_whileimpl_894 {
() => {
// Module: crate::stream::stream::take_while
// Provides: {"impl_894"}
// Dependencies: {}
impl < St , Fut , F > TakeWhile < St , Fut , F > where St : Stream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_taking : false } } delegate_access_inner ! (stream , St , ()) ; }
};
}
