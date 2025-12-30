// Generated macro for impl_864 (impl)
macro_rules! Depcrate_strategy_justimpl_864 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_864"}
// Dependencies: {}
impl < T : fmt :: Debug > Strategy for fn () -> T { type Tree = Self ; type Value = T ; fn new_tree (& self , _ : & mut TestRunner) -> NewTree < Self > { Ok (* self) } }
};
}
