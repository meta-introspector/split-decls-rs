macro_rules! deps {
    () => {
        TypeParam!();
        LifetimeParam!();
        GenericParam!();
        ConstParam!();
    };
}

macro_rules! macro_363 {
    () => {
        deps!();
        impl_from ! (TypeParam , ConstParam , LifetimeParam for GenericParam) ;
    };
}

macro_363!()