macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_1390 {
    () => {
        deps!();
        impl < T > Copy for SendPtr < T > { }
    };
}

impl_1390!()