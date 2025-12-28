macro_rules! deps {
    () => {
        ModuleId!();
        FieldId!();
        AdtId!();
        GenericParamId!();
        AttrDefId!();
        MacroId!();
    };
}

macro_rules! macro_718 {
    () => {
        deps!();
        impl_from ! (ModuleId , FieldId , AdtId (StructId , EnumId , UnionId) , EnumVariantId , StaticId , ConstId , FunctionId , TraitId , TypeAliasId , MacroId (Macro2Id , MacroRulesId , ProcMacroId) , ImplId , GenericParamId , ExternCrateId , UseId for AttrDefId) ;
    };
}

macro_718!()