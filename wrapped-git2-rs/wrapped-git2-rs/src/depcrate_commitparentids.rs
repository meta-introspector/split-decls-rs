// Generated macro for ParentIds (struct)
macro_rules! Depcrate_commitParentIds {
() => {
// Module: crate::commit
// Provides: {"ParentIds"}
// Dependencies: {}
# [doc = " An iterator over the parent commits' ids of a commit."] # [doc = ""] # [doc = " Aborts iteration when a commit cannot be found"] pub struct ParentIds < 'commit > { range : Range < usize > , commit : & 'commit Commit < 'commit > , }
};
}
