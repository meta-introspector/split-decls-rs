// Generated macro for walk_impl_item_ref (function)
macro_rules! Depcrate_intravisitwalk_impl_item_ref {
() => {
// Module: crate::intravisit
// Provides: {"walk_impl_item_ref"}
// Dependencies: {}
pub fn walk_impl_item_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , id : ImplItemId) -> V :: Result { visitor . visit_nested_impl_item (id) }
};
}
