// Generated macro for impl_891 (impl)
macro_rules! Depcrate_strategy_mapimpl_891 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_891"}
// Dependencies: {}
impl < S : fmt :: Debug , O > fmt :: Debug for MapInto < S , O > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MapInto") . field ("source" , & self . source) . finish () } }
};
}
