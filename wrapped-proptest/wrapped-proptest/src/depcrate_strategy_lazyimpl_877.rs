// Generated macro for impl_877 (impl)
macro_rules! Depcrate_strategy_lazyimpl_877 {
() => {
// Module: crate::strategy::lazy
// Provides: {"impl_877"}
// Dependencies: {}
impl < S : Strategy > fmt :: Debug for LazyValueTreeState < S > where S :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { LazyValueTreeState :: Initialized (value_tree) => { f . debug_tuple ("Initialized") . field (value_tree) . finish () } LazyValueTreeState :: Uninitialized { strategy , .. } => f . debug_struct ("Uninitialized") . field ("strategy" , strategy) . finish () , LazyValueTreeState :: Failed => write ! (f , "Failed") , } } }
};
}
