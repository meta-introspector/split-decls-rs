macro_rules! deps {
    () => {
        ConstParam!();
        TypeParam!();
        LifetimeParam!();
        GenericParam!();
    };
}

macro_rules! macro_363 {
    () => {
        deps!();
        impl_from ! (TypeParam , ConstParam , LifetimeParam for GenericParam) ;
    };
}

macro_363!();