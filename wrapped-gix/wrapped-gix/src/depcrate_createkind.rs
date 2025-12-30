// Generated macro for Kind (enum)
macro_rules! Depcrate_createKind {
() => {
// Module: crate::create
// Provides: {"Kind"}
// Dependencies: {}
# [doc = " The kind of repository to create."] # [derive (Debug , Copy , Clone)] pub enum Kind { # [doc = " An empty repository with a `.git` folder, setup to contain files in its worktree."] WithWorktree , # [doc = " A bare repository without a worktree."] Bare , }
};
}
