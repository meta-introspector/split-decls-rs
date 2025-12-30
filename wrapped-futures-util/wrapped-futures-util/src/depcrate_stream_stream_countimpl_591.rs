// Generated macro for impl_591 (impl)
macro_rules! Depcrate_stream_stream_countimpl_591 {
() => {
// Module: crate::stream::stream::count
// Provides: {"impl_591"}
// Dependencies: {}
impl < St > fmt :: Debug for Count < St > where St : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Count") . field ("stream" , & self . stream) . field ("count" , & self . count) . finish () } }
};
}
