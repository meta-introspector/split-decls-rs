// Generated macro for Ignore (enum)
macro_rules! Depcrate_configIgnore {
() => {
// Module: crate::config
// Provides: {"Ignore"}
// Dependencies: {}
# [doc = " Determine how the submodule participates in `git status` queries. This setting also affects `git diff`."] # [derive (Default , Debug , Clone , Copy , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum Ignore { # [doc = " Submodule changes won't be considered at all, which is the fastest option."] All , # [doc = " Ignore any changes to the submodule working tree, only show committed differences between the `HEAD` of the submodule"] # [doc = " compared to the recorded commit in the superproject."] Dirty , # [doc = " Only ignore untracked files in the submodule, but show modifications to the submodule working tree as well as differences"] # [doc = " between the recorded commit in the superproject and the checked-out commit in the submodule."] Untracked , # [doc = " No modifications to the submodule are ignored, which shows untracked files, modified files in the submodule worktree as well as"] # [doc = " differences between the recorded commit in the superproject and the checked-out commit in the submodule."] # [default] None , }
};
}
