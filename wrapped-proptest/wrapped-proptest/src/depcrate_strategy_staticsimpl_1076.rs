// Generated macro for impl_1076 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1076 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1076"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for Map < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Map") . field ("source" , & self . source) . field ("fun" , & "<function>") . finish () } }
};
}
