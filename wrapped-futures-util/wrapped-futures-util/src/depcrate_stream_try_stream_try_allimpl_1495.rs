// Generated macro for impl_1495 (impl)
macro_rules! Depcrate_stream_try_stream_try_allimpl_1495 {
() => {
// Module: crate::stream::try_stream::try_all
// Provides: {"impl_1495"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for TryAll < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryAll") . field ("stream" , & self . stream) . field ("done" , & self . done) . field ("future" , & self . future) . finish () } }
};
}
