// Generated macro for Path (struct)
macro_rules! Depcrate_rt_pathPath {
() => {
// Module: crate::rt::path
// Provides: {"Path"}
// Dependencies: {}
# [doc = " An execution path"] # [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) struct Path { preemption_bound : Option < u8 > , # [doc = " Current execution's position in the branches vec."] # [doc = ""] # [doc = " When the execution starts, this is zero, but `branches` might not be"] # [doc = " empty."] # [doc = ""] # [doc = " In order to perform an exhaustive search, the execution is seeded with a"] # [doc = " set of branches."] pos : usize , # [doc = " List of all branches in the execution."] # [doc = ""] # [doc = " A branch is of type `Schedule`, `Load`, or `Spurious`"] branches : object :: Store < Entry > , # [doc = " If true, exploring is enabled at start"] exploring : bool , # [doc = " If true, the user decided to skip the current execution branch. We do"] # [doc = " not do any further exploration here."] skipping : bool , # [doc = " How to reset the `exploring` state"] exploring_on_start : bool , }
};
}
