// Generated macro for is_trait_impl_item (function)
macro_rules! Depcrateis_trait_impl_item {
() => {
// Module: crate
// Provides: {"is_trait_impl_item"}
// Dependencies: {}
# [doc = " Check if parent of a hir node is a trait implementation block."] # [doc = " For example, `f` in"] # [doc = " ```no_run"] # [doc = " # struct S;"] # [doc = " # trait Trait { fn f(); }"] # [doc = " impl Trait for S {"] # [doc = "     fn f() {}"] # [doc = " }"] # [doc = " ```"] pub fn is_trait_impl_item (cx : & LateContext < '_ > , hir_id : HirId) -> bool { if let Node :: Item (item) = cx . tcx . parent_hir_node (hir_id) { matches ! (item . kind , ItemKind :: Impl (Impl { of_trait : Some (_) , .. })) } else { false } }
};
}
