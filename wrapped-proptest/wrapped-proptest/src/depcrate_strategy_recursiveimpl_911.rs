// Generated macro for impl_911 (impl)
macro_rules! Depcrate_strategy_recursiveimpl_911 {
() => {
// Module: crate::strategy::recursive
// Provides: {"impl_911"}
// Dependencies: {}
impl < T , F > Clone for Recursive < T , F > { fn clone (& self) -> Self { Recursive { base : self . base . clone () , recurse : Arc :: clone (& self . recurse) , depth : self . depth , desired_size : self . desired_size , expected_branch_size : self . expected_branch_size , } } }
};
}
