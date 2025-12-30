// Generated macro for Allocs (struct)
macro_rules! Depcrate_souper_harvestAllocs {
() => {
// Module: crate::souper_harvest
// Provides: {"Allocs"}
// Dependencies: {}
# [doc = " Allocations that we reuse across many LHS candidate harvests."] # [derive (Default)] struct Allocs { # [doc = " A map from cranelift IR to souper IR for values that we've already"] # [doc = " translated into souper IR."] ir_to_souper_val : HashMap < ir :: Value , ast :: ValueId > , # [doc = " Stack of to-visit and to-trace values for the post-order DFS."] dfs_stack : Vec < StackEntry > , # [doc = " Set of values we've already seen in our post-order DFS."] dfs_seen : HashSet < ir :: Value > , }
};
}
