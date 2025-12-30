// Generated macro for impl_835 (impl)
macro_rules! Depcrate_strategy_flattenimpl_835 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_835"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for IndFlattenMap < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("IndFlattenMap") . field ("source" , & self . source) . field ("fun" , & "<function>") . finish () } }
};
}
