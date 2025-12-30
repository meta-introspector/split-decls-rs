// Generated macro for impl_887 (impl)
macro_rules! Depcrate_strategy_mapimpl_887 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_887"}
// Dependencies: {}
impl < S : Strategy , O : fmt :: Debug , F : Fn (S :: Value) -> O > Strategy for Map < S , F > { type Tree = Map < S :: Tree , F > ; type Value = O ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . source . new_tree (runner) . map (| v | Map { source : v , fun : Arc :: clone (& self . fun) , }) } }
};
}
