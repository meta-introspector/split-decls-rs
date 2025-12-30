// Generated macro for impl_1104 (impl)
macro_rules! Depcrate_stream_stream_for_each_concurrentimpl_1104 {
() => {
// Module: crate::stream::stream::for_each_concurrent
// Provides: {"impl_1104"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for ForEachConcurrent < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ForEachConcurrent") . field ("stream" , & self . stream) . field ("futures" , & self . futures) . field ("limit" , & self . limit) . finish () } }
};
}
