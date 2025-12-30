// Generated macro for impl_859 (impl)
macro_rules! Depcrate_strategy_justimpl_859 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_859"}
// Dependencies: {}
impl < T : fmt :: Debug , F : Clone + Fn () -> T > Strategy for LazyJust < T , F > { type Tree = Self ; type Value = T ; fn new_tree (& self , _ : & mut TestRunner) -> NewTree < Self > { Ok (self . clone ()) } }
};
}
