// Generated macro for impl_918 (impl)
macro_rules! Depcrateimpl_918 {
() => {
// Module: crate
// Provides: {"impl_918"}
// Dependencies: {}
impl TryFrom < ModuleDefId > for AttrDefId { type Error = () ; fn try_from (value : ModuleDefId) -> Result < Self , Self :: Error > { match value { ModuleDefId :: ModuleId (it) => Ok (it . into ()) , ModuleDefId :: FunctionId (it) => Ok (it . into ()) , ModuleDefId :: AdtId (it) => Ok (it . into ()) , ModuleDefId :: EnumVariantId (it) => Ok (it . into ()) , ModuleDefId :: ConstId (it) => Ok (it . into ()) , ModuleDefId :: StaticId (it) => Ok (it . into ()) , ModuleDefId :: TraitId (it) => Ok (it . into ()) , ModuleDefId :: TypeAliasId (it) => Ok (it . into ()) , ModuleDefId :: MacroId (id) => Ok (id . into ()) , ModuleDefId :: BuiltinType (_) => Err (()) , } } }
};
}
