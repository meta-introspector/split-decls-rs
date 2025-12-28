macro_rules! deps {
    () => {
        Tuple1Combination!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < I > From < I > for Tuple1Combination < I > { fn from (iter : I) -> Self { Self { iter } } }
    };
}

impl_103!();