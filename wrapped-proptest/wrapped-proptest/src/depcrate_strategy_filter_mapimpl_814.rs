// Generated macro for impl_814 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_814 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_814"}
// Dependencies: {}
impl < V : fmt :: Debug , F , O > fmt :: Debug for FilterMapValueTree < V , F , O > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FilterMapValueTree") . field ("source" , & self . source) . field ("current" , & "<current>") . field ("fun" , & "<function>") . finish () } }
};
}
