macro_rules! deps {
    () => {
        Pointable!();
        Atomic!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Default for Atomic < T > { fn default () -> Self { Self :: null () } }
    };
}

impl_26!()