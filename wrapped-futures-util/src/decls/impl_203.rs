macro_rules! deps {
    () => {
        AlwaysReady!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < T , F : Fn () -> T + Copy > Copy for AlwaysReady < T , F > { }
    };
}

impl_203!();