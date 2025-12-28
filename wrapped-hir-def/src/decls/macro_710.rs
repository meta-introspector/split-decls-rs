macro_rules! deps {
    () => {
        GenericDefId!();
        AdtId!();
    };
}

macro_rules! macro_710 {
    () => {
        deps!();
        impl_from ! (AdtId (StructId , EnumId , UnionId) , ConstId , FunctionId , ImplId , StaticId , TraitId , TypeAliasId for GenericDefId) ;
    };
}

macro_710!()