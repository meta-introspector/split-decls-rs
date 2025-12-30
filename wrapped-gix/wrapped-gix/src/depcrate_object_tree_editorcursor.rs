// Generated macro for Cursor (struct)
macro_rules! Depcrate_object_tree_editorCursor {
() => {
// Module: crate::object::tree::editor
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " A cursor at a specific portion of a tree to [edit](super::Editor)."] pub struct Cursor < 'a , 'repo > { inner : gix_object :: tree :: editor :: Cursor < 'a , 'repo > , validate : gix_validate :: path :: component :: Options , repo : & 'repo Repository , }
};
}
