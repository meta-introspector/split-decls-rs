// Generated macro for tree_diff_at_file_path (function)
macro_rules! Depcrate_file_functiontree_diff_at_file_path {
() => {
// Module: crate::file::function
// Provides: {"tree_diff_at_file_path"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] fn tree_diff_at_file_path (odb : impl gix_object :: Find + gix_object :: FindHeader , file_path : & BStr , id : ObjectId , parent_id : ObjectId , cache : Option < & gix_commitgraph :: Graph > , stats : & mut Statistics , state : & mut gix_diff :: tree :: State , resource_cache : & mut gix_diff :: blob :: Platform , commit_buf : & mut Vec < u8 > , lhs_tree_buf : & mut Vec < u8 > , rhs_tree_buf : & mut Vec < u8 > , rewrites : Option < gix_diff :: Rewrites > ,) -> Result < Option < TreeDiffChange > , Error > { let parent_tree_id = find_commit (cache , & odb , & parent_id , commit_buf) ? . tree_id () ? ; let parent_tree_iter = odb . find_tree_iter (& parent_tree_id , lhs_tree_buf) ? ; stats . trees_decoded += 1 ; let tree_id = find_commit (cache , & odb , & id , commit_buf) ? . tree_id () ? ; let tree_iter = odb . find_tree_iter (& tree_id , rhs_tree_buf) ? ; stats . trees_decoded += 1 ; let result = tree_diff_without_rewrites_at_file_path (& odb , file_path , stats , state , parent_tree_iter , tree_iter) ? ; if matches ! (result , Some (TreeDiffChange :: Modification { .. })) { return Ok (result) ; } let Some (rewrites) = rewrites else { return Ok (result) ; } ; let result = tree_diff_with_rewrites_at_file_path (& odb , file_path , stats , state , resource_cache , parent_tree_iter , tree_iter , rewrites ,) ? ; Ok (result) }
};
}
