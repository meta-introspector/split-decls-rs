// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_optionimpl_1325 {
() => {
// Module: crate::option
// Provides: {"impl_1325"}
// Dependencies: {}
impl < T : fmt :: Debug > Strategy for NoneStrategy < T > { type Tree = Self ; type Value = Option < T > ; fn new_tree (& self , _ : & mut TestRunner) -> NewTree < Self > { Ok (* self) } }
};
}
