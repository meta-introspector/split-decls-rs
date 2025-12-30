// Generated macro for impl_885 (impl)
macro_rules! Depcrate_strategy_mapimpl_885 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_885"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for Map < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Map") . field ("source" , & self . source) . field ("fun" , & "<function>") . finish () } }
};
}
