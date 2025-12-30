// Generated macro for Recursive (struct)
macro_rules! Depcrate_strategy_recursiveRecursive {
() => {
// Module: crate::strategy::recursive
// Provides: {"Recursive"}
// Dependencies: {}
# [doc = " Return type from `Strategy::prop_recursive()`."] # [must_use = "strategies do nothing unless used"] pub struct Recursive < T , F > { base : BoxedStrategy < T > , recurse : Arc < F > , depth : u32 , desired_size : u32 , expected_branch_size : u32 , }
};
}
