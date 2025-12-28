macro_rules! deps {
    () => {
        ValueTyDefId!();
    };
}

macro_rules! macro_813 {
    () => {
        deps!();
        impl_from ! (FunctionId , StructId , UnionId , EnumVariantId , ConstId , StaticId for ValueTyDefId) ;
    };
}

macro_813!()