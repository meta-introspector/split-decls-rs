macro_rules! deps {
    () => {
        AdtId!();
        ModuleDefId!();
        MacroId!();
        AttrDefId!();
        BuiltinType!();
        ModuleId!();
    };
}

macro_rules! impl_719 {
    () => {
        deps!();
        impl TryFrom < ModuleDefId > for AttrDefId { type Error = () ; fn try_from (value : ModuleDefId) -> Result < Self , Self :: Error > { match value { ModuleDefId :: ModuleId (it) => Ok (it . into ()) , ModuleDefId :: FunctionId (it) => Ok (it . into ()) , ModuleDefId :: AdtId (it) => Ok (it . into ()) , ModuleDefId :: EnumVariantId (it) => Ok (it . into ()) , ModuleDefId :: ConstId (it) => Ok (it . into ()) , ModuleDefId :: StaticId (it) => Ok (it . into ()) , ModuleDefId :: TraitId (it) => Ok (it . into ()) , ModuleDefId :: TypeAliasId (it) => Ok (it . into ()) , ModuleDefId :: MacroId (id) => Ok (id . into ()) , ModuleDefId :: BuiltinType (_) => Err (()) , } } }
    };
}

impl_719!()