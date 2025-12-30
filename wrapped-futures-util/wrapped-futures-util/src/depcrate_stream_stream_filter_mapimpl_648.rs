// Generated macro for impl_648 (impl)
macro_rules! Depcrate_stream_stream_filter_mapimpl_648 {
() => {
// Module: crate::stream::stream::filter_map
// Provides: {"impl_648"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for FilterMap < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FilterMap") . field ("stream" , & self . stream) . field ("pending" , & self . pending) . finish () } }
};
}
