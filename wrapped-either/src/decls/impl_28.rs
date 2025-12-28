macro_rules! deps {
    () => {
        IntoEither!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > IntoEither for T { }
    };
}

impl_28!()