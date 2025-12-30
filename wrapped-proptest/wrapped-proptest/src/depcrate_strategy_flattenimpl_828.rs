// Generated macro for impl_828 (impl)
macro_rules! Depcrate_strategy_flattenimpl_828 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_828"}
// Dependencies: {}
impl < S : ValueTree > Clone for FlattenValueTree < S > where S :: Value : Strategy + Clone , S : Clone , < S :: Value as Strategy > :: Tree : Clone , { fn clone (& self) -> Self { FlattenValueTree { meta : self . meta . clone () , current : self . current . clone () , final_complication : self . final_complication . clone () , runner : self . runner . clone () , complicate_regen_remaining : self . complicate_regen_remaining , } } }
};
}
