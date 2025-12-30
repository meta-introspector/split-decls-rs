// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_stream_stream_try_for_each_concurrentimpl_1146 {
() => {
// Module: crate::stream::stream::try_for_each_concurrent
// Provides: {"impl_1146"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for TryForEachConcurrent < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryForEachConcurrent") . field ("stream" , & self . stream) . field ("futures" , & self . futures) . field ("limit" , & self . limit) . finish () } }
};
}
