macro_rules! deps {
    () => {
        LifetimeParam!();
        TypeParam!();
        ConstParam!();
        GenericParam!();
    };
}

macro_rules! macro_6 {
    () => {
        deps!();
        impl_has_attrs_enum ! [TypeParam , ConstParam , LifetimeParam for GenericParam] ;
    };
}

macro_6!();