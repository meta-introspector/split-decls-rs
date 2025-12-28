macro_rules! deps {
    () => {
        TypeAlias!();
        Trait!();
        Function!();
        Static!();
        Union!();
        Struct!();
    };
}

macro_rules! LangItemTarget {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum LangItemTarget { EnumId (EnumId) , Function (FunctionId) , ImplDef (ImplId) , Static (StaticId) , Struct (StructId) , Union (UnionId) , TypeAlias (TypeAliasId) , Trait (TraitId) , EnumVariant (EnumVariantId) , }
    };
}

LangItemTarget!()