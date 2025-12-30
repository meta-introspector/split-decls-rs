// Generated macro for impl_55 (impl)
macro_rules! Depcrate_file_functionimpl_55 {
() => {
// Module: crate::file::function
// Provides: {"impl_55"}
// Dependencies: {}
impl From < gix_diff :: tree_with_rewrites :: Change > for TreeDiffChange { fn from (value : gix_diff :: tree_with_rewrites :: Change) -> Self { use gix_diff :: tree_with_rewrites :: Change ; match value { Change :: Addition { id , .. } => Self :: Addition { id } , Change :: Deletion { .. } => Self :: Deletion , Change :: Modification { previous_id , id , .. } => Self :: Modification { previous_id , id } , Change :: Rewrite { source_location , source_id , id , .. } => Self :: Rewrite { source_location , source_id , id , } , } } }
};
}
