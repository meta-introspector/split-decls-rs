// Generated macro for WorktreeLockStatus (enum)
macro_rules! Depcrate_worktreeWorktreeLockStatus {
() => {
// Module: crate::worktree
// Provides: {"WorktreeLockStatus"}
// Dependencies: {}
# [doc = " Lock Status of a worktree"] # [derive (PartialEq , Debug)] pub enum WorktreeLockStatus { # [doc = " Worktree is Unlocked"] Unlocked , # [doc = " Worktree is locked with the optional message"] Locked (Option < String >) , }
};
}
