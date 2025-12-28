macro_rules! deps {
    () => {
        Stride!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'a , A > Copy for Stride < 'a , A > { }
    };
}

impl_105!();