// Generated macro for impl_34 (impl)
macro_rules! Depcrate_from_idimpl_34 {
() => {
// Module: crate::from_id
// Provides: {"impl_34"}
// Dependencies: {}
impl From < ModuleDefId > for ModuleDef { fn from (id : ModuleDefId) -> Self { match id { ModuleDefId :: ModuleId (it) => ModuleDef :: Module (it . into ()) , ModuleDefId :: FunctionId (it) => ModuleDef :: Function (it . into ()) , ModuleDefId :: AdtId (it) => ModuleDef :: Adt (it . into ()) , ModuleDefId :: EnumVariantId (it) => ModuleDef :: Variant (it . into ()) , ModuleDefId :: ConstId (it) => ModuleDef :: Const (it . into ()) , ModuleDefId :: StaticId (it) => ModuleDef :: Static (it . into ()) , ModuleDefId :: TraitId (it) => ModuleDef :: Trait (it . into ()) , ModuleDefId :: TypeAliasId (it) => ModuleDef :: TypeAlias (it . into ()) , ModuleDefId :: BuiltinType (it) => ModuleDef :: BuiltinType (it . into ()) , ModuleDefId :: MacroId (it) => ModuleDef :: Macro (it . into ()) , } } }
};
}
