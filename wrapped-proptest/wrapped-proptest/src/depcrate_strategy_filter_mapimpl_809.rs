// Generated macro for impl_809 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_809 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_809"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for FilterMap < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FilterMap") . field ("source" , & self . source) . field ("whence" , & self . whence) . field ("fun" , & "<function>") . finish () } }
};
}
