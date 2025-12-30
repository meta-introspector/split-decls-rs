// Generated macro for convert_module_item_kind (function)
macro_rules! Depcrate_arbitrary_source_item_orderingconvert_module_item_kind {
() => {
// Module: crate::arbitrary_source_item_ordering
// Provides: {"convert_module_item_kind"}
// Dependencies: {}
# [doc = " Converts a [`rustc_hir::ItemKind`] to a"] # [doc = " [`SourceItemOrderingModuleItemKind`]."] # [doc = ""] # [doc = " This is implemented here because `rustc_hir` is not a dependency of"] # [doc = " `clippy_config`."] fn convert_module_item_kind (value : & ItemKind < '_ >) -> SourceItemOrderingModuleItemKind { # [allow (clippy :: enum_glob_use)] use SourceItemOrderingModuleItemKind :: * ; match value { ItemKind :: ExternCrate (..) => ExternCrate , ItemKind :: Use (..) => Use , ItemKind :: Static (..) => Static , ItemKind :: Const (..) => Const , ItemKind :: Fn { .. } => Fn , ItemKind :: Macro (..) => Macro , ItemKind :: Mod (..) => Mod , ItemKind :: ForeignMod { .. } => ForeignMod , ItemKind :: GlobalAsm { .. } => GlobalAsm , ItemKind :: TyAlias (..) => TyAlias , ItemKind :: Enum (..) => Enum , ItemKind :: Struct (..) => Struct , ItemKind :: Union (..) => Union , ItemKind :: Trait (..) => Trait , ItemKind :: TraitAlias (..) => TraitAlias , ItemKind :: Impl (..) => Impl , } }
};
}
