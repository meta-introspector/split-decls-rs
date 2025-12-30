// Generated macro for join_single_use_tree (function)
macro_rules! Depcrate_join_linesjoin_single_use_tree {
() => {
// Module: crate::join_lines
// Provides: {"join_single_use_tree"}
// Dependencies: {}
fn join_single_use_tree (edit : & mut TextEditBuilder , token : & SyntaxToken) -> Option < () > { let use_tree_list = ast :: UseTreeList :: cast (token . parent () ?) ? ; let (tree ,) = use_tree_list . use_trees () . collect_tuple () ? ; edit . replace (use_tree_list . syntax () . text_range () , tree . syntax () . text () . to_string ()) ; Some (()) }
};
}
