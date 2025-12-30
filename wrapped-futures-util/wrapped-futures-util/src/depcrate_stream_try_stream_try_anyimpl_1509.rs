// Generated macro for impl_1509 (impl)
macro_rules! Depcrate_stream_try_stream_try_anyimpl_1509 {
() => {
// Module: crate::stream::try_stream::try_any
// Provides: {"impl_1509"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for TryAny < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryAny") . field ("stream" , & self . stream) . field ("done" , & self . done) . field ("future" , & self . future) . finish () } }
};
}
