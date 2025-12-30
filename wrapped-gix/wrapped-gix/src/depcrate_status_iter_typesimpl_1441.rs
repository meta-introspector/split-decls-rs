// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_status_iter_typesimpl_1441 {
() => {
// Module: crate::status::iter::types
// Provides: {"impl_1441"}
// Dependencies: {}
# [doc = " Access"] impl Item { # [doc = " Return the relative path at which the item can currently be found in the working tree or index."] pub fn location (& self) -> & BStr { match self { Item :: IndexWorktree (change) => change . rela_path () , Item :: TreeIndex (change) => change . location () , } } }
};
}
