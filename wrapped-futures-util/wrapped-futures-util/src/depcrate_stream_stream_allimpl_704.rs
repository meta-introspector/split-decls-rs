// Generated macro for impl_704 (impl)
macro_rules! Depcrate_stream_stream_allimpl_704 {
() => {
// Module: crate::stream::stream::all
// Provides: {"impl_704"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for All < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("All") . field ("stream" , & self . stream) . field ("done" , & self . done) . field ("future" , & self . future) . finish () } }
};
}
