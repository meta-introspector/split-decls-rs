// Generated macro for impl_1069 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1069 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1069"}
// Dependencies: {}
impl < S : fmt :: Debug , F > fmt :: Debug for Filter < S , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Filter") . field ("source" , & self . source) . field ("whence" , & self . whence) . field ("fun" , & "<function>") . finish () } }
};
}
