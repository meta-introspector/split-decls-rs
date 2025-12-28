macro_rules! deps {
    () => {
        Params!();
        ParamsIter!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'a > ParamsIter < 'a > { fn new (params : & 'a Params) -> Self { Self { params , index : 0 } } }
    };
}

impl_6!();