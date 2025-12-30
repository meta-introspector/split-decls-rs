// Generated macro for impl_940 (impl)
macro_rules! Depcrate_stream_stream_try_for_eachimpl_940 {
() => {
// Module: crate::stream::stream::try_for_each
// Provides: {"impl_940"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for TryForEach < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryForEach") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
};
}
