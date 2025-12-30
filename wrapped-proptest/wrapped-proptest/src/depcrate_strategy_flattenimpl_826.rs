// Generated macro for impl_826 (impl)
macro_rules! Depcrate_strategy_flattenimpl_826 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_826"}
// Dependencies: {}
impl < S : Strategy > Strategy for Flatten < S > where S :: Value : Strategy , { type Tree = FlattenValueTree < S :: Tree > ; type Value = < S :: Value as Strategy > :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let meta = self . source . new_tree (runner) ? ; FlattenValueTree :: new (runner , meta) } }
};
}
