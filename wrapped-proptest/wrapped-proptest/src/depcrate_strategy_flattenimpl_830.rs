// Generated macro for impl_830 (impl)
macro_rules! Depcrate_strategy_flattenimpl_830 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_830"}
// Dependencies: {}
impl < S : ValueTree > FlattenValueTree < S > where S :: Value : Strategy , { fn new (runner : & mut TestRunner , meta : S) -> Result < Self , Reason > { let current = meta . current () . new_tree (runner) ? ; Ok (FlattenValueTree { meta : Fuse :: new (meta) , current : Fuse :: new (current) , final_complication : None , runner : runner . partial_clone () , complicate_regen_remaining : 0 , }) } }
};
}
