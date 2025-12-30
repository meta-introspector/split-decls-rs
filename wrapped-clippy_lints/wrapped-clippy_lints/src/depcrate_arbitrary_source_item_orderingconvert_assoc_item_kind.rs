// Generated macro for convert_assoc_item_kind (function)
macro_rules! Depcrate_arbitrary_source_item_orderingconvert_assoc_item_kind {
() => {
// Module: crate::arbitrary_source_item_ordering
// Provides: {"convert_assoc_item_kind"}
// Dependencies: {}
# [doc = " Converts a [`ty::AssocKind`] to a [`SourceItemOrderingTraitAssocItemKind`]."] # [doc = ""] # [doc = " This is implemented here because `rustc_hir` is not a dependency of"] # [doc = " `clippy_config`."] fn convert_assoc_item_kind (cx : & LateContext < '_ > , owner_id : OwnerId) -> SourceItemOrderingTraitAssocItemKind { # [allow (clippy :: enum_glob_use)] use SourceItemOrderingTraitAssocItemKind :: * ; let kind = cx . tcx . associated_item (owner_id . def_id) . kind ; match kind { AssocKind :: Const { .. } => Const , AssocKind :: Type { .. } => Type , AssocKind :: Fn { .. } => Fn , } }
};
}
