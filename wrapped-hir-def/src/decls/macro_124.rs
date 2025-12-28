macro_rules! deps {
    () => {
        LifetimeParamId!();
        TypeParamId!();
        GenericParamId!();
        ConstParamId!();
    };
}

macro_rules! macro_124 {
    () => {
        deps!();
        impl_from ! (TypeParamId , LifetimeParamId , ConstParamId for GenericParamId) ;
    };
}

macro_124!()