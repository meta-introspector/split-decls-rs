macro_rules! deps {
    () => {
        NoHashHasher!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T > Copy for NoHashHasher < T > { }
    };
}

impl_8!()