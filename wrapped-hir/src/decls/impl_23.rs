macro_rules! deps {
    () => {
        GenericParam!();
        ConstParam!();
        LifetimeParam!();
        TypeParam!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < GenericParam > for GenericParamId { fn from (id : GenericParam) -> Self { match id { GenericParam :: LifetimeParam (it) => GenericParamId :: LifetimeParamId (it . id) , GenericParam :: ConstParam (it) => GenericParamId :: ConstParamId (it . id) , GenericParam :: TypeParam (it) => GenericParamId :: TypeParamId (it . id) , } } }
    };
}

impl_23!()