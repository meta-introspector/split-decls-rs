macro_rules! deps {
    () => {
        ExternCrateDecl!();
        Adt!();
        Module!();
        Impl!();
        Function!();
        Macro!();
        Trait!();
        Static!();
        Const!();
        TypeAlias!();
        Variant!();
        GenericParam!();
        Field!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        impl_has_attrs ! [(Field , FieldId) , (Variant , EnumVariantId) , (Static , StaticId) , (Const , ConstId) , (Trait , TraitId) , (TypeAlias , TypeAliasId) , (Macro , MacroId) , (Function , FunctionId) , (Adt , AdtId) , (Module , ModuleId) , (GenericParam , GenericParamId) , (Impl , ImplId) , (ExternCrateDecl , ExternCrateId) ,] ;
    };
}

macro_3!()