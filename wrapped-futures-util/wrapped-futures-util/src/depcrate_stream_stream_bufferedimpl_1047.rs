// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_stream_stream_bufferedimpl_1047 {
() => {
// Module: crate::stream::stream::buffered
// Provides: {"impl_1047"}
// Dependencies: {}
impl < St > fmt :: Debug for Buffered < St > where St : Stream + fmt :: Debug , St :: Item : Future , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Buffered") . field ("stream" , & self . stream) . field ("in_progress_queue" , & self . in_progress_queue) . field ("max" , & self . max) . finish () } }
};
}
