// Generated macro for impl_54 (impl)
macro_rules! Depcrate_file_functionimpl_54 {
() => {
// Module: crate::file::function
// Provides: {"impl_54"}
// Dependencies: {}
impl From < gix_diff :: tree :: recorder :: Change > for TreeDiffChange { fn from (value : gix_diff :: tree :: recorder :: Change) -> Self { use gix_diff :: tree :: recorder :: Change ; match value { Change :: Addition { oid , .. } => Self :: Addition { id : oid } , Change :: Deletion { .. } => Self :: Deletion , Change :: Modification { previous_oid , oid , .. } => Self :: Modification { previous_id : previous_oid , id : oid , } , } } }
};
}
