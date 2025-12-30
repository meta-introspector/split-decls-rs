// Generated macro for TreeDiffChange (enum)
macro_rules! Depcrate_file_functionTreeDiffChange {
() => {
// Module: crate::file::function
// Provides: {"TreeDiffChange"}
// Dependencies: {}
# [doc = " The union of [`gix_diff::tree::recorder::Change`] and [`gix_diff::tree_with_rewrites::Change`],"] # [doc = " keeping only the blame-relevant information."] enum TreeDiffChange { Addition { id : ObjectId , } , Deletion , Modification { previous_id : ObjectId , id : ObjectId , } , Rewrite { source_location : BString , source_id : ObjectId , id : ObjectId , } , }
};
}
