// Generated macro for span_and_hid_of_item_alike_node (function)
macro_rules! Depcrate_undocumented_unsafe_blocksspan_and_hid_of_item_alike_node {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"span_and_hid_of_item_alike_node"}
// Dependencies: {}
fn span_and_hid_of_item_alike_node (node : & Node < '_ >) -> Option < (Span , HirId) > { match node { Node :: Item (item) => Some ((item . span , item . owner_id . into ())) , Node :: TraitItem (ti) => Some ((ti . span , ti . owner_id . into ())) , Node :: ImplItem (ii) => Some ((ii . span , ii . owner_id . into ())) , _ => None , } }
};
}
