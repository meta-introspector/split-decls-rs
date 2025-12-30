// Generated macro for Item (enum)
macro_rules! Depcrate_status_iter_typesItem {
() => {
// Module: crate::status::iter::types
// Provides: {"Item"}
// Dependencies: {}
# [doc = " The item produced by the [iterator](Iter)."] # [derive (Clone , PartialEq , Debug)] pub enum Item { # [doc = " A change between the index and the worktree."] # [doc = ""] # [doc = " Note that untracked changes are also collected here."] IndexWorktree (index_worktree :: Item) , # [doc = " A change between the three of `HEAD` and the index."] TreeIndex (gix_diff :: index :: Change) , }
};
}
