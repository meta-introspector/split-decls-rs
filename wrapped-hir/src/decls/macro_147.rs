macro_rules! deps {
    () => {
        TypeParam!();
        ConstParam!();
        GenericParam!();
        LifetimeParam!();
    };
}

macro_rules! macro_147 {
    () => {
        deps!();
        impl_from ! (TypeParam , ConstParam , LifetimeParam for GenericParam) ;
    };
}

macro_147!()