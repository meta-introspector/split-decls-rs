macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : Primitive > From < MaybeUninit < T > > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from a potentially uninitialized value."] # [inline] fn from (v : MaybeUninit < T >) -> Self { Self :: new (v) } }
    };
}

impl_6!()