// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1083 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1083"}
// Dependencies: {}
impl < St , Fc > fmt :: Debug for FlattenUnorderedWithFlowController < St , Fc > where St : Stream + fmt :: Debug , St :: Item : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlattenUnorderedWithFlowController") . field ("poll_state" , & self . poll_state) . field ("inner_streams" , & self . inner_streams) . field ("limit" , & self . limit) . field ("stream" , & self . stream) . field ("is_stream_done" , & self . is_stream_done) . field ("flow_controller" , & self . flow_controller) . finish () } }
};
}
