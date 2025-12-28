macro_rules! deps {
    () => {
        Atomic!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > From < T > for Atomic < T > { fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_29!();