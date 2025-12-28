macro_rules! deps {
    () => {
        AdtId!();
        BuiltinType!();
        ModuleId!();
        TypeParamId!();
    };
}

macro_rules! TypeNs {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum TypeNs { SelfType (ImplId) , GenericParam (TypeParamId) , AdtId (AdtId) , AdtSelfType (AdtId) , EnumVariantId (EnumVariantId) , TypeAliasId (TypeAliasId) , BuiltinType (BuiltinType) , TraitId (TraitId) , ModuleId (ModuleId) , }
    };
}

TypeNs!();