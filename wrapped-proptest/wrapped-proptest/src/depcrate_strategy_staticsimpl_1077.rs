// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1077 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1077"}
// Dependencies: {}
impl < S : Strategy , F : Clone + MapFn < S :: Value > > Strategy for Map < S , F > { type Tree = Map < S :: Tree , F > ; type Value = F :: Output ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . source . new_tree (runner) . map (| v | Map { source : v , fun : self . fun . clone () , }) } }
};
}
