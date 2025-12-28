macro_rules! deps {
    () => {
        SplitInclusive!();
    };
}

macro_rules! impl_1313 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > SplitInclusive < 'ch , P > { fn new (chars : & 'ch str , separator : P) -> Self { SplitInclusive { chars , separator } } }
    };
}

impl_1313!()