macro_rules! deps {
    () => {
        Atomic!();
        Owned!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > From < Box < T > > for Atomic < T > { fn from (b : Box < T >) -> Self { Self :: from (Owned :: from (b)) } }
    };
}

impl_28!()