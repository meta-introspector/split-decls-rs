// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_stream_try_stream_or_elseimpl_1225 {
() => {
// Module: crate::stream::try_stream::or_else
// Provides: {"impl_1225"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for OrElse < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OrElse") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
};
}
