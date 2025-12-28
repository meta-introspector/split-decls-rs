macro_rules! deps {
    () => {
        ModuleId!();
        AdtId!();
        TypeParamId!();
        BuiltinType!();
    };
}

macro_rules! TypeNs {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum TypeNs { SelfType (ImplId) , GenericParam (TypeParamId) , AdtId (AdtId) , AdtSelfType (AdtId) , EnumVariantId (EnumVariantId) , TypeAliasId (TypeAliasId) , BuiltinType (BuiltinType) , TraitId (TraitId) , ModuleId (ModuleId) , }
    };
}

TypeNs!()