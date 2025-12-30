// Generated macro for StashApplyProgressCb (type)
macro_rules! Depcrate_stashStashApplyProgressCb {
() => {
// Module: crate::stash
// Provides: {"StashApplyProgressCb"}
// Dependencies: {}
# [doc = " Stash application progress notification function."] # [doc = ""] # [doc = " Return `true` to continue processing, or `false` to"] # [doc = " abort the stash application."] pub type StashApplyProgressCb < 'a > = dyn FnMut (StashApplyProgress) -> bool + 'a ;
};
}
