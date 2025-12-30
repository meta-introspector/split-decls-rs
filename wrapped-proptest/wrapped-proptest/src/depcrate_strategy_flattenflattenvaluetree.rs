// Generated macro for FlattenValueTree (struct)
macro_rules! Depcrate_strategy_flattenFlattenValueTree {
() => {
// Module: crate::strategy::flatten
// Provides: {"FlattenValueTree"}
// Dependencies: {}
# [doc = " The `ValueTree` produced by `Flatten`."] pub struct FlattenValueTree < S : ValueTree > where S :: Value : Strategy , { meta : Fuse < S > , current : Fuse < < S :: Value as Strategy > :: Tree > , final_complication : Option < Fuse < < S :: Value as Strategy > :: Tree > > , runner : TestRunner , complicate_regen_remaining : u32 , }
};
}
