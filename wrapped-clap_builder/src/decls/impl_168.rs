macro_rules! deps {
    () => {
        Resettable!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T > From < T > for Resettable < T > { fn from (other : T) -> Self { Self :: Value (other) } }
    };
}

impl_168!()