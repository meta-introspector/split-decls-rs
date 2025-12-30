// Generated macro for impl_876 (impl)
macro_rules! Depcrate_strategy_lazyimpl_876 {
() => {
// Module: crate::strategy::lazy
// Provides: {"impl_876"}
// Dependencies: {}
impl < S : Strategy > Clone for LazyValueTreeState < S > where S :: Tree : Clone , { fn clone (& self) -> Self { use LazyValueTreeState :: * ; match self { Initialized (v) => Initialized (v . clone ()) , Uninitialized { strategy , runner } => Uninitialized { strategy : Arc :: clone (strategy) , runner : runner . clone () , } , Failed => Failed , } } }
};
}
