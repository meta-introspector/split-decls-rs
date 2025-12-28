macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T > Copy for SendPtr < T > { }
    };
}

impl_23!()