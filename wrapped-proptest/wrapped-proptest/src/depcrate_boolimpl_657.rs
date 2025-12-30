// Generated macro for impl_657 (impl)
macro_rules! Depcrate_boolimpl_657 {
() => {
// Module: crate::bool
// Provides: {"impl_657"}
// Dependencies: {}
impl Strategy for Any { type Tree = BoolValueTree ; type Value = bool ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (BoolValueTree :: new (runner . rng () . random ())) } }
};
}
