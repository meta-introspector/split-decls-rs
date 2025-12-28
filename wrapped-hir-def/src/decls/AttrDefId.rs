macro_rules! deps {
    () => {
        AdtId!();
        FieldId!();
        GenericParamId!();
        MacroId!();
        ModuleId!();
    };
}

macro_rules! AttrDefId {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum AttrDefId { ModuleId (ModuleId) , FieldId (FieldId) , AdtId (AdtId) , FunctionId (FunctionId) , EnumVariantId (EnumVariantId) , StaticId (StaticId) , ConstId (ConstId) , TraitId (TraitId) , TypeAliasId (TypeAliasId) , MacroId (MacroId) , ImplId (ImplId) , GenericParamId (GenericParamId) , ExternBlockId (ExternBlockId) , ExternCrateId (ExternCrateId) , UseId (UseId) , }
    };
}

AttrDefId!();