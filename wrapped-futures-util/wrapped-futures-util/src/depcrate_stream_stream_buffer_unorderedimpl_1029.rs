// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_stream_stream_buffer_unorderedimpl_1029 {
() => {
// Module: crate::stream::stream::buffer_unordered
// Provides: {"impl_1029"}
// Dependencies: {}
impl < St > fmt :: Debug for BufferUnordered < St > where St : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufferUnordered") . field ("stream" , & self . stream) . field ("in_progress_queue" , & self . in_progress_queue) . field ("max" , & self . max) . finish () } }
};
}
