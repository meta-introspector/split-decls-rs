// Generated macro for impl_732 (impl)
macro_rules! Depcrate_stream_stream_for_eachimpl_732 {
() => {
// Module: crate::stream::stream::for_each
// Provides: {"impl_732"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for ForEach < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ForEach") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
};
}
