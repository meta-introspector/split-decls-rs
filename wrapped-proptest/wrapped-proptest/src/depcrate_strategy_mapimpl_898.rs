// Generated macro for impl_898 (impl)
macro_rules! Depcrate_strategy_mapimpl_898 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_898"}
// Dependencies: {}
impl < S : Strategy , O : fmt :: Debug , F : Fn (S :: Value , TestRng) -> O > Strategy for Perturb < S , F > { type Tree = PerturbValueTree < S :: Tree , F > ; type Value = O ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let rng = runner . new_rng () ; self . source . new_tree (runner) . map (| source | PerturbValueTree { source , rng , fun : Arc :: clone (& self . fun) , }) } }
};
}
