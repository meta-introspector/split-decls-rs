// Generated macro for impl_988 (impl)
macro_rules! Depcrate_strategy_traitsimpl_988 {
() => {
// Module: crate::strategy::traits
// Provides: {"impl_988"}
// Dependencies: {}
impl < T : fmt :: Debug > Strategy for BoxedStrategy < T > { type Tree = BoxedVT < T > ; type Value = T ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . 0 . new_tree (runner) } fn boxed (self) -> BoxedStrategy < Self :: Value > where Self : Sized + 'static , { self } }
};
}
