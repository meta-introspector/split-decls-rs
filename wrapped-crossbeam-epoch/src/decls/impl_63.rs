macro_rules! deps {
    () => {
        Shared!();
        Pointable!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Default for Shared < '_ , T > { fn default () -> Self { Self :: null () } }
    };
}

impl_63!();