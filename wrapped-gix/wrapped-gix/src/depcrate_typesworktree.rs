// Generated macro for Worktree (struct)
macro_rules! Depcrate_typesWorktree {
() => {
// Module: crate::types
// Provides: {"Worktree"}
// Dependencies: {}
# [doc = " A worktree checkout containing the files of the repository in consumable form."] # [derive (Debug , Clone)] pub struct Worktree < 'repo > { pub (crate) parent : & 'repo Repository , # [doc = " The root path of the checkout."] pub (crate) path : & 'repo std :: path :: Path , }
};
}
