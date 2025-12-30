// Generated macro for RawVisibility (enum)
macro_rules! Depcrate_item_treeRawVisibility {
() => {
// Module: crate::item_tree
// Provides: {"RawVisibility"}
// Dependencies: {}
# [doc = " Visibility of an item, not yet resolved."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum RawVisibility { # [doc = " `pub(in module)`, `pub(crate)` or `pub(super)`. Also private, which is"] # [doc = " equivalent to `pub(self)`."] Module (Interned < ModPath > , VisibilityExplicitness) , # [doc = " `pub(self)`."] PubSelf (VisibilityExplicitness) , # [doc = " `pub(crate)`."] PubCrate , # [doc = " `pub`."] Public , }
};
}
