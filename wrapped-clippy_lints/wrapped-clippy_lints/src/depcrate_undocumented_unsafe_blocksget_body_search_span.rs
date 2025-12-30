// Generated macro for get_body_search_span (function)
macro_rules! Depcrate_undocumented_unsafe_blocksget_body_search_span {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"get_body_search_span"}
// Dependencies: {}
fn get_body_search_span (cx : & LateContext < '_ >) -> Option < Span > { let body = cx . enclosing_body ? ; let mut maybe_mod_item = None ; for (_ , parent_node) in cx . tcx . hir_parent_iter (body . hir_id) { match parent_node { Node :: Crate (mod_) => return Some (mod_ . spans . inner_span) , Node :: Item (hir :: Item { kind : ItemKind :: Mod (_ , mod_) , span , .. }) => { return maybe_mod_item . and_then (| item | comment_start_before_item_in_mod (cx , mod_ , * span , & item)) . map (| comment_start | mod_ . spans . inner_span . with_lo (comment_start . into ())) . or (Some (* span)) ; } , node if let Some ((span , _)) = span_and_hid_of_item_alike_node (& node) && ! is_const_or_static (& node) => { return Some (span) ; } , Node :: Item (item) => { maybe_mod_item = Some (* item) ; } , _ => { maybe_mod_item = None ; } , } } None }
};
}
