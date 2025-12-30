// Generated macro for Parents (enum)
macro_rules! Depcrate_commitParents {
() => {
// Module: crate::commit
// Provides: {"Parents"}
// Dependencies: {}
# [doc = " Specify how to handle commit parents during traversal."] # [derive (Default , Copy , Clone)] pub enum Parents { # [doc = " Traverse all parents, useful for traversing the entire ancestry."] # [default] All , # [doc = " Only traverse along the first parent, which commonly ignores all branches."] First , }
};
}
