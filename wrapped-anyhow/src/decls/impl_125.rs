macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'a , T > Copy for Ref < 'a , T > where T : ? Sized { }
    };
}

impl_125!();