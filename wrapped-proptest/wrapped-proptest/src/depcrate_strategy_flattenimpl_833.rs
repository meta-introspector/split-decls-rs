// Generated macro for impl_833 (impl)
macro_rules! Depcrate_strategy_flattenimpl_833 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_833"}
// Dependencies: {}
impl < S : Strategy > Strategy for IndFlatten < S > where S :: Value : Strategy , { type Tree = < S :: Value as Strategy > :: Tree ; type Value = < S :: Value as Strategy > :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let inner = self . 0 . new_tree (runner) ? ; inner . current () . new_tree (runner) } }
};
}
