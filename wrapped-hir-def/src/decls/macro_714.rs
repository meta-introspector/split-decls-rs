macro_rules! deps {
    () => {
        CallableDefId!();
    };
}

macro_rules! macro_714 {
    () => {
        deps!();
        impl_from ! (FunctionId , StructId , EnumVariantId for CallableDefId) ;
    };
}

macro_714!()