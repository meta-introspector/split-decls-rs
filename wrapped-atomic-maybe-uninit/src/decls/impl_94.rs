macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
        Primitive!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < T : Primitive > From < T > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from an initialized value."] # [inline] fn from (v : T) -> Self { Self :: new (MaybeUninit :: new (v)) } }
    };
}

impl_94!();