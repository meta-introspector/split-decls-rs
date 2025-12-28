macro_rules! deps {
    () => {
        FutureExt!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < F : Future + ? Sized > FutureExt for F { }
    };
}

impl_37!();