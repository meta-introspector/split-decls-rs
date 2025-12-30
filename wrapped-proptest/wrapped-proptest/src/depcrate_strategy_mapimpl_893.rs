// Generated macro for impl_893 (impl)
macro_rules! Depcrate_strategy_mapimpl_893 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_893"}
// Dependencies: {}
impl < S : Strategy , O : fmt :: Debug > Strategy for MapInto < S , O > where S :: Value : Into < O > , { type Tree = MapInto < S :: Tree , O > ; type Value = O ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . source . new_tree (runner) . map (MapInto :: new) } }
};
}
