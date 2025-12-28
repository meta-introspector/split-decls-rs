macro_rules! deps {
    () => {
        TypeParam!();
        GenericParam!();
        ConstParam!();
        LifetimeParam!();
    };
}

macro_rules! macro_6 {
    () => {
        deps!();
        impl_has_attrs_enum ! [TypeParam , ConstParam , LifetimeParam for GenericParam] ;
    };
}

macro_6!()