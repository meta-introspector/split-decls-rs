macro_rules! deps {
    () => {
        IntoFallible!();
        Iterator!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T , I : iter :: Iterator < Item = T > > From < I > for IntoFallible < I > { fn from (value : I) -> Self { Self (value) } }
    };
}

impl_72!()