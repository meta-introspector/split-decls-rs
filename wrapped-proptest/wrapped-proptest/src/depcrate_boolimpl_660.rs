// Generated macro for impl_660 (impl)
macro_rules! Depcrate_boolimpl_660 {
() => {
// Module: crate::bool
// Provides: {"impl_660"}
// Dependencies: {}
impl Strategy for Weighted { type Tree = BoolValueTree ; type Value = bool ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (BoolValueTree :: new (runner . rng () . random_bool (self . 0))) } }
};
}
