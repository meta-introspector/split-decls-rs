// Generated macro for walk_trait_item_ref (function)
macro_rules! Depcrate_intravisitwalk_trait_item_ref {
() => {
// Module: crate::intravisit
// Provides: {"walk_trait_item_ref"}
// Dependencies: {}
pub fn walk_trait_item_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , id : TraitItemId) -> V :: Result { visitor . visit_nested_trait_item (id) }
};
}
