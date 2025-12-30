// Generated macro for impl_1192 (impl)
macro_rules! Depcrate_stream_try_stream_and_thenimpl_1192 {
() => {
// Module: crate::stream::try_stream::and_then
// Provides: {"impl_1192"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for AndThen < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AndThen") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
};
}
