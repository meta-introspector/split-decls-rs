// Generated macro for HirTyCtxt (trait)
macro_rules! Depcrate_intravisitHirTyCtxt {
() => {
// Module: crate::intravisit
// Provides: {"HirTyCtxt"}
// Dependencies: {}
# [doc = " HIR things retrievable from `TyCtxt`, avoiding an explicit dependence on"] # [doc = " `TyCtxt`. The only impls are for `!` (where these functions are never"] # [doc = " called) and `TyCtxt` (in `rustc_middle`)."] pub trait HirTyCtxt < 'hir > { # [doc = " Retrieves the `Node` corresponding to `id`."] fn hir_node (& self , hir_id : HirId) -> Node < 'hir > ; fn hir_body (& self , id : BodyId) -> & 'hir Body < 'hir > ; fn hir_item (& self , id : ItemId) -> & 'hir Item < 'hir > ; fn hir_trait_item (& self , id : TraitItemId) -> & 'hir TraitItem < 'hir > ; fn hir_impl_item (& self , id : ImplItemId) -> & 'hir ImplItem < 'hir > ; fn hir_foreign_item (& self , id : ForeignItemId) -> & 'hir ForeignItem < 'hir > ; }
};
}
