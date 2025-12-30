// Generated macro for impl_542 (impl)
macro_rules! Depcrate_repository_objectimpl_542 {
() => {
// Module: crate::repository::object
// Provides: {"impl_542"}
// Dependencies: {}
# [doc = " Tree editing"] # [cfg (feature = "tree-editor")] impl crate :: Repository { # [doc = " Return an editor for adjusting the tree at `id`."] # [doc = ""] # [doc = " This can be the [empty tree id](ObjectId::empty_tree) to build a tree from scratch."] # [doc (alias = "treebuilder" , alias = "git2")] pub fn edit_tree (& self , id : impl Into < ObjectId > ,) -> Result < object :: tree :: Editor < '_ > , crate :: repository :: edit_tree :: Error > { let tree = self . find_tree (id) ? ; Ok (tree . edit () ?) } }
};
}
