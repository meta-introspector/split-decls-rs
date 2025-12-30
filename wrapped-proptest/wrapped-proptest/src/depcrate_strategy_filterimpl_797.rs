// Generated macro for impl_797 (impl)
macro_rules! Depcrate_strategy_filterimpl_797 {
() => {
// Module: crate::strategy::filter
// Provides: {"impl_797"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for Filter < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Filter") . field ("source" , & self . source) . field ("whence" , & self . whence) . field ("fun" , & "<function>") . finish () } }
};
}
