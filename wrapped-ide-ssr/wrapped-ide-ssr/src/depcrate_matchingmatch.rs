// Generated macro for Match (struct)
macro_rules! Depcrate_matchingMatch {
() => {
// Module: crate::matching
// Provides: {"Match"}
// Dependencies: {}
# [doc = " Information about a match that was found."] # [derive (Debug)] pub struct Match { pub (crate) range : FileRange , pub (crate) matched_node : SyntaxNode , pub (crate) placeholder_values : FxHashMap < Var , PlaceholderMatch > , pub (crate) ignored_comments : Vec < ast :: Comment > , pub (crate) rule_index : usize , # [doc = " The depth of matched_node."] pub (crate) depth : usize , pub (crate) rendered_template_paths : FxHashMap < SyntaxNode , hir :: ModPath > , }
};
}
