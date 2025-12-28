macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > From < T > for AtomicCell < T > { # [inline] fn from (val : T) -> Self { Self :: new (val) } }
    };
}

impl_40!()