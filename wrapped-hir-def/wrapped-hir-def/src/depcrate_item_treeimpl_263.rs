// Generated macro for impl_263 (impl)
macro_rules! Depcrate_item_treeimpl_263 {
() => {
// Module: crate::item_tree
// Provides: {"impl_263"}
// Dependencies: {}
impl Use { # [doc = " Expands the `UseTree` into individually imported `ModPath`s."] pub fn expand (& self , mut cb : impl FnMut (Idx < ast :: UseTree > , ModPath , ImportKind , Option < ImportAlias >) ,) { self . use_tree . expand_impl (None , & mut 0 , & mut cb) } }
};
}
