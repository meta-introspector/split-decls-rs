// Generated macro for impl_925 (impl)
macro_rules! Depcrate_stream_stream_thenimpl_925 {
() => {
// Module: crate::stream::stream::then
// Provides: {"impl_925"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for Then < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Then") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
};
}
