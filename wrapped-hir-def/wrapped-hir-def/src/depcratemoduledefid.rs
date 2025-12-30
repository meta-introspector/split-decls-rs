// Generated macro for ModuleDefId (enum)
macro_rules! DepcrateModuleDefId {
() => {
// Module: crate
// Provides: {"ModuleDefId"}
// Dependencies: {}
# [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDefId { ModuleId (ModuleId) , FunctionId (FunctionId) , AdtId (AdtId) , EnumVariantId (EnumVariantId) , ConstId (ConstId) , StaticId (StaticId) , TraitId (TraitId) , TypeAliasId (TypeAliasId) , BuiltinType (BuiltinType) , MacroId (MacroId) , }
};
}
