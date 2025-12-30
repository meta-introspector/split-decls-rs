// Generated macro for Commit (struct)
macro_rules! Depcrate_file_commitCommit {
() => {
// Module: crate::file::commit
// Provides: {"Commit"}
// Dependencies: {}
# [doc = " A commit as stored in a [`File`]."] # [derive (Copy , Clone)] pub struct Commit < 'a > { file : & 'a File , pos : file :: Position , commit_timestamp : u64 , generation : u32 , parent1 : ParentEdge , parent2 : ParentEdge , root_tree_id : & 'a gix_hash :: oid , }
};
}
