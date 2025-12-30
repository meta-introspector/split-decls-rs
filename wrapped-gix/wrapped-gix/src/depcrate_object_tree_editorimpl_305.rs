// Generated macro for impl_305 (impl)
macro_rules! Depcrate_object_tree_editorimpl_305 {
() => {
// Module: crate::object::tree::editor
// Provides: {"impl_305"}
// Dependencies: {}
# [doc = " Tree editing"] # [cfg (feature = "tree-editor")] impl < 'repo > crate :: Tree < 'repo > { # [doc = " Start editing a new tree based on this one."] # [doc (alias = "treebuilder" , alias = "git2")] pub fn edit (& self) -> Result < super :: Editor < 'repo > , init :: Error > { super :: Editor :: new (self) } }
};
}
