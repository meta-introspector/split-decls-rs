macro_rules! deps {
    () => {
        Impl!();
        ExternCrateDecl!();
        Static!();
        Adt!();
        Trait!();
        Field!();
        Const!();
        TypeAlias!();
        Macro!();
        Function!();
        Module!();
        Variant!();
        GenericParam!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        impl_has_attrs ! [(Field , FieldId) , (Variant , EnumVariantId) , (Static , StaticId) , (Const , ConstId) , (Trait , TraitId) , (TypeAlias , TypeAliasId) , (Macro , MacroId) , (Function , FunctionId) , (Adt , AdtId) , (Module , ModuleId) , (GenericParam , GenericParamId) , (Impl , ImplId) , (ExternCrateDecl , ExternCrateId) ,] ;
    };
}

macro_3!();