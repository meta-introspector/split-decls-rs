// Generated macro for impl_925 (impl)
macro_rules! Depcrateimpl_925 {
() => {
// Module: crate
// Provides: {"impl_925"}
// Dependencies: {}
impl TryFrom < ModuleDefId > for AttrDefId { type Error = () ; fn try_from (value : ModuleDefId) -> Result < Self , Self :: Error > { match value { ModuleDefId :: ModuleId (it) => Ok (it . into ()) , ModuleDefId :: FunctionId (it) => Ok (it . into ()) , ModuleDefId :: AdtId (it) => Ok (it . into ()) , ModuleDefId :: EnumVariantId (it) => Ok (it . into ()) , ModuleDefId :: ConstId (it) => Ok (it . into ()) , ModuleDefId :: StaticId (it) => Ok (it . into ()) , ModuleDefId :: TraitId (it) => Ok (it . into ()) , ModuleDefId :: TypeAliasId (it) => Ok (it . into ()) , ModuleDefId :: TraitAliasId (id) => Ok (id . into ()) , ModuleDefId :: MacroId (id) => Ok (id . into ()) , ModuleDefId :: BuiltinType (_) => Err (()) , } } }
};
}
