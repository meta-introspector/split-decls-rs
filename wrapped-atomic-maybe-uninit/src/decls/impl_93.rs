macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
        Primitive!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < T : Primitive > From < MaybeUninit < T > > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from a potentially uninitialized value."] # [inline] fn from (v : MaybeUninit < T >) -> Self { Self :: new (v) } }
    };
}

impl_93!()