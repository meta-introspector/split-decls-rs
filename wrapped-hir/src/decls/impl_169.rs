macro_rules! deps {
    () => {
        GenericArgKind!();
        Type!();
        Const!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl GenericArgKind { fn from_id (id : GenericParamId) -> Self { match id { GenericParamId :: TypeParamId (_) => GenericArgKind :: Type , GenericParamId :: ConstParamId (_) => GenericArgKind :: Const , GenericParamId :: LifetimeParamId (_) => GenericArgKind :: Lifetime , } } }
    };
}

impl_169!()