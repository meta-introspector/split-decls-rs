macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_1282 {
    () => {
        deps!();
        impl < T > From < T > for Mutex < T > { fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_1282!();