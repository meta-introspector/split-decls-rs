macro_rules! deps {
    () => {
        Owned!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > From < T > for Owned < T > { fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_43!();