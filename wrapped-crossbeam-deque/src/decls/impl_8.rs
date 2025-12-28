macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T > Copy for Buffer < T > { }
    };
}

impl_8!();