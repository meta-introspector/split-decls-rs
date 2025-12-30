// Generated macro for DEFAULT_MODULE_ITEM_ORDERING_GROUPS (const)
macro_rules! Depcrate_confDEFAULT_MODULE_ITEM_ORDERING_GROUPS {
() => {
// Module: crate::conf
// Provides: {"DEFAULT_MODULE_ITEM_ORDERING_GROUPS"}
// Dependencies: {}
const DEFAULT_MODULE_ITEM_ORDERING_GROUPS : & [(& str , & [SourceItemOrderingModuleItemKind])] = { # [allow (clippy :: enum_glob_use)] use SourceItemOrderingModuleItemKind :: * ; & [("modules" , & [ExternCrate , Mod , ForeignMod]) , ("use" , & [Use]) , ("macros" , & [Macro]) , ("global_asm" , & [GlobalAsm]) , ("UPPER_SNAKE_CASE" , & [Static , Const]) , ("PascalCase" , & [TyAlias , Enum , Struct , Union , Trait , TraitAlias , Impl]) , ("lower_snake_case" , & [Fn]) ,] } ;
};
}
