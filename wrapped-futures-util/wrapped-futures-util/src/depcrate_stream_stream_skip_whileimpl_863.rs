// Generated macro for impl_863 (impl)
macro_rules! Depcrate_stream_stream_skip_whileimpl_863 {
() => {
// Module: crate::stream::stream::skip_while
// Provides: {"impl_863"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for SkipWhile < St , Fut , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SkipWhile") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . field ("done_skipping" , & self . done_skipping) . finish () } }
};
}
