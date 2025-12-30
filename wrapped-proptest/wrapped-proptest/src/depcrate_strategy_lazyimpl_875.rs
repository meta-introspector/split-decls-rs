// Generated macro for impl_875 (impl)
macro_rules! Depcrate_strategy_lazyimpl_875 {
() => {
// Module: crate::strategy::lazy
// Provides: {"impl_875"}
// Dependencies: {}
impl < S : Strategy > fmt :: Debug for LazyValueTree < S > where S :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("LazyValueTree") . field ("state" , & self . state) . finish () } }
};
}
