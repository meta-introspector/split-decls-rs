macro_rules! deps {
    () => {
        AlwaysReady!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < T , F : Fn () -> T > Unpin for AlwaysReady < T , F > { }
    };
}

impl_204!();