// Generated macro for impl_900 (impl)
macro_rules! Depcrate_strategy_mapimpl_900 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_900"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for PerturbValueTree < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("PerturbValueTree") . field ("source" , & self . source) . field ("fun" , & "<function>") . field ("rng" , & self . rng) . finish () } }
};
}
