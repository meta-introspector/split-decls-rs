// Generated macro for SourceItemOrderingModuleItemKind (enum)
macro_rules! Depcrate_typesSourceItemOrderingModuleItemKind {
() => {
// Module: crate::types
// Provides: {"SourceItemOrderingModuleItemKind"}
// Dependencies: {}
# [doc = " Represents the items that can occur within a module."] # [derive (Clone , Debug , Deserialize , Eq , Hash , PartialEq , Serialize)] # [serde (rename_all = "snake_case")] pub enum SourceItemOrderingModuleItemKind { ExternCrate , Mod , ForeignMod , Use , Macro , GlobalAsm , Static , Const , TyAlias , Enum , Struct , Union , Trait , TraitAlias , Impl , Fn , }
};
}
