// Generated macro for comment_start_before_item_in_mod (function)
macro_rules! Depcrate_undocumented_unsafe_blockscomment_start_before_item_in_mod {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"comment_start_before_item_in_mod"}
// Dependencies: {}
fn comment_start_before_item_in_mod (cx : & LateContext < '_ > , parent_mod : & hir :: Mod < '_ > , parent_mod_span : Span , item : & hir :: Item < '_ > ,) -> Option < CommentStartBeforeItem > { parent_mod . item_ids . iter () . enumerate () . find_map (| (idx , item_id) | { if * item_id == item . item_id () { if idx == 0 { if let Some (sp) = walk_span_to_context (parent_mod_span , SyntaxContext :: root ()) { return Some (CommentStartBeforeItem :: Offset (sp . lo ())) ; } } else { let prev_item = cx . tcx . hir_item (parent_mod . item_ids [idx - 1]) ; if prev_item . span . is_dummy () { return Some (CommentStartBeforeItem :: Start) ; } if let Some (sp) = walk_span_to_context (prev_item . span , SyntaxContext :: root ()) { return Some (CommentStartBeforeItem :: Offset (sp . hi ())) ; } } } None }) }
};
}
