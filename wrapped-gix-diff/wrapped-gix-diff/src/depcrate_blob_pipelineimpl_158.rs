// Generated macro for impl_158 (impl)
macro_rules! Depcrate_blob_pipelineimpl_158 {
() => {
// Module: crate::blob::pipeline
// Provides: {"impl_158"}
// Dependencies: {}
# [doc = " Lifecycle"] impl Pipeline { # [doc = " Create a new instance of a pipeline which produces blobs suitable for diffing. `roots` allow to read worktree files directly, otherwise"] # [doc = " `worktree_filter` is used to transform object database data directly. `drivers` further configure individual paths."] # [doc = " `options` are used to further configure the way we act.."] pub fn new (roots : WorktreeRoots , worktree_filter : gix_filter :: Pipeline , mut drivers : Vec < super :: Driver > , options : Options ,) -> Self { drivers . sort_by (| a , b | a . name . cmp (& b . name)) ; Pipeline { roots , worktree_filter , drivers , options , attrs : { let mut out = gix_filter :: attributes :: search :: Outcome :: default () ; out . initialize_with_selection (& Default :: default () , Some ("diff")) ; out } , path : Default :: default () , } } }
};
}
