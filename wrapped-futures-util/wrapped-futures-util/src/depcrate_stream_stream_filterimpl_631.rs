// Generated macro for impl_631 (impl)
macro_rules! Depcrate_stream_stream_filterimpl_631 {
() => {
// Module: crate::stream::stream::filter
// Provides: {"impl_631"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for Filter < St , Fut , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Filter") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . finish () } }
};
}
