macro_rules! deps {
    () => {
        CallableDefId!();
    };
}

macro_rules! macro_142 {
    () => {
        deps!();
        impl_from ! (FunctionId , StructId , EnumVariantId for CallableDefId) ;
    };
}

macro_142!()