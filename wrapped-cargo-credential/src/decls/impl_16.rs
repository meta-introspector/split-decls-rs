macro_rules! deps {
    () => {
        Secret!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T > From < T > for Secret < T > { fn from (inner : T) -> Self { Self { inner } } }
    };
}

impl_16!()