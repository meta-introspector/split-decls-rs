macro_rules! deps {
    () => {
        GenericParamId!();
        TypeParamId!();
        ConstParamId!();
        LifetimeParamId!();
    };
}

macro_rules! macro_696 {
    () => {
        deps!();
        impl_from ! (TypeParamId , LifetimeParamId , ConstParamId for GenericParamId) ;
    };
}

macro_696!();