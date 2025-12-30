// Generated macro for impl_854 (impl)
macro_rules! Depcrate_strategy_justimpl_854 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_854"}
// Dependencies: {}
impl < T : Clone + fmt :: Debug > Strategy for Just < T > { type Tree = Self ; type Value = T ; fn new_tree (& self , _ : & mut TestRunner) -> NewTree < Self > { Ok (self . clone ()) } }
};
}
