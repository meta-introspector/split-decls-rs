macro_rules! deps {
    () => {
        TyDefId!();
    };
}

macro_rules! macro_129 {
    () => {
        deps!();
        impl_from ! (BuiltinType , AdtId (StructId , EnumId , UnionId) , TypeAliasId for TyDefId) ;
    };
}

macro_129!();