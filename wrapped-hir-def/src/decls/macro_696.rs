macro_rules! deps {
    () => {
        LifetimeParamId!();
        TypeParamId!();
        ConstParamId!();
        GenericParamId!();
    };
}

macro_rules! macro_696 {
    () => {
        deps!();
        impl_from ! (TypeParamId , LifetimeParamId , ConstParamId for GenericParamId) ;
    };
}

macro_696!()