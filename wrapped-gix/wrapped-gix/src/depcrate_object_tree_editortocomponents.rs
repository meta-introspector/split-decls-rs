// Generated macro for ToComponents (trait)
macro_rules! Depcrate_object_tree_editorToComponents {
() => {
// Module: crate::object::tree::editor
// Provides: {"ToComponents"}
// Dependencies: {}
# [doc = " Obtain an iterator over `BStr`-components."] # [doc = ""] # [doc = " Note that the implementation is simple, and it's mainly meant for statically known strings"] # [doc = " or locations obtained during a merge."] pub trait ToComponents { # [doc = " Return an iterator over the components of a path, without the separator."] fn to_components (& self) -> impl Iterator < Item = & BStr > ; }
};
}
