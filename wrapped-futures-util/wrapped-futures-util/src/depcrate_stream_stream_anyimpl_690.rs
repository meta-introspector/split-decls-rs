// Generated macro for impl_690 (impl)
macro_rules! Depcrate_stream_stream_anyimpl_690 {
() => {
// Module: crate::stream::stream::any
// Provides: {"impl_690"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for Any < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Any") . field ("stream" , & self . stream) . field ("done" , & self . done) . field ("future" , & self . future) . finish () } }
};
}
