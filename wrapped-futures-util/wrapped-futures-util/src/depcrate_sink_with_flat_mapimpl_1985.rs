// Generated macro for impl_1985 (impl)
macro_rules! Depcrate_sink_with_flat_mapimpl_1985 {
() => {
// Module: crate::sink::with_flat_map
// Provides: {"impl_1985"}
// Dependencies: {}
impl < Si , Item , U , St , F > fmt :: Debug for WithFlatMap < Si , Item , U , St , F > where Si : fmt :: Debug , St : fmt :: Debug , Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WithFlatMap") . field ("sink" , & self . sink) . field ("stream" , & self . stream) . field ("buffer" , & self . buffer) . finish () } }
};
}
