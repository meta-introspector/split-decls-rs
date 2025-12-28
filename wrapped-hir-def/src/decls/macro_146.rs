macro_rules! deps {
    () => {
        AttrDefId!();
        FieldId!();
        AdtId!();
        GenericParamId!();
        MacroId!();
        ModuleId!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        impl_from ! (ModuleId , FieldId , AdtId (StructId , EnumId , UnionId) , EnumVariantId , StaticId , ConstId , FunctionId , TraitId , TypeAliasId , MacroId (Macro2Id , MacroRulesId , ProcMacroId) , ImplId , GenericParamId , ExternCrateId , UseId for AttrDefId) ;
    };
}

macro_146!()