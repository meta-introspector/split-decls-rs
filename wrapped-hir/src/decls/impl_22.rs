macro_rules! deps {
    () => {
        LifetimeParam!();
        TypeParam!();
        ConstParam!();
        GenericParam!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < GenericParamId > for GenericParam { fn from (id : GenericParamId) -> Self { match id { GenericParamId :: TypeParamId (it) => GenericParam :: TypeParam (it . into ()) , GenericParamId :: ConstParamId (it) => GenericParam :: ConstParam (it . into ()) , GenericParamId :: LifetimeParamId (it) => GenericParam :: LifetimeParam (it . into ()) , } } }
    };
}

impl_22!();