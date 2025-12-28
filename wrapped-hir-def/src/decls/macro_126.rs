macro_rules! deps {
    () => {
        ModuleId!();
        MacroId!();
        ModuleDefId!();
        AdtId!();
    };
}

macro_rules! macro_126 {
    () => {
        deps!();
        impl_from ! (MacroId (Macro2Id , MacroRulesId , ProcMacroId) , ModuleId , FunctionId , AdtId (StructId , EnumId , UnionId) , EnumVariantId , ConstId , StaticId , TraitId , TypeAliasId , BuiltinType for ModuleDefId) ;
    };
}

macro_126!()