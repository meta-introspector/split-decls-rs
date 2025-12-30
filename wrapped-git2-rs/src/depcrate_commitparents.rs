// Generated macro for Parents (struct)
macro_rules! Depcrate_commitParents {
() => {
// Module: crate::commit
// Provides: {"Parents"}
// Dependencies: {}
# [doc = " An iterator over the parent commits of a commit."] # [doc = ""] # [doc = " Aborts iteration when a commit cannot be found"] pub struct Parents < 'commit , 'repo > { range : Range < usize > , commit : & 'commit Commit < 'repo > , }
};
}
