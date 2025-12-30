// Generated macro for walk_foreign_item_ref (function)
macro_rules! Depcrate_intravisitwalk_foreign_item_ref {
() => {
// Module: crate::intravisit
// Provides: {"walk_foreign_item_ref"}
// Dependencies: {}
pub fn walk_foreign_item_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , id : ForeignItemId) -> V :: Result { visitor . visit_nested_foreign_item (id) }
};
}
