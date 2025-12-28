macro_rules! deps {
    () => {
        BuiltinType!();
        ModuleDefId!();
        MacroId!();
        ModuleId!();
        AdtId!();
    };
}

macro_rules! macro_698 {
    () => {
        deps!();
        impl_from ! (MacroId (Macro2Id , MacroRulesId , ProcMacroId) , ModuleId , FunctionId , AdtId (StructId , EnumId , UnionId) , EnumVariantId , ConstId , StaticId , TraitId , TypeAliasId , BuiltinType for ModuleDefId) ;
    };
}

macro_698!()