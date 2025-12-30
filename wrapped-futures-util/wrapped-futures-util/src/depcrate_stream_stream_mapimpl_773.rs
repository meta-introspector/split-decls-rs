// Generated macro for impl_773 (impl)
macro_rules! Depcrate_stream_stream_mapimpl_773 {
() => {
// Module: crate::stream::stream::map
// Provides: {"impl_773"}
// Dependencies: {}
impl < St , F > fmt :: Debug for Map < St , F > where St : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Map") . field ("stream" , & self . stream) . finish () } }
};
}
