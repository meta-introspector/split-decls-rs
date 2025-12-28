macro_rules! deps {
    () => {
        ValueTyDefId!();
    };
}

macro_rules! macro_131 {
    () => {
        deps!();
        impl_from ! (FunctionId , StructId , UnionId , EnumVariantId , ConstId , StaticId for ValueTyDefId) ;
    };
}

macro_131!()