macro_rules! deps {
    () => {
        TyDefId!();
    };
}

macro_rules! macro_811 {
    () => {
        deps!();
        impl_from ! (BuiltinType , AdtId (StructId , EnumId , UnionId) , TypeAliasId for TyDefId) ;
    };
}

macro_811!()