macro_rules! deps {
    () => {
        BuiltinType!();
        MacroId!();
        ModuleId!();
        AdtId!();
    };
}

macro_rules! ModuleDefId {
    () => {
        deps!();
        # [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDefId { ModuleId (ModuleId) , FunctionId (FunctionId) , AdtId (AdtId) , EnumVariantId (EnumVariantId) , ConstId (ConstId) , StaticId (StaticId) , TraitId (TraitId) , TypeAliasId (TypeAliasId) , BuiltinType (BuiltinType) , MacroId (MacroId) , }
    };
}

ModuleDefId!();