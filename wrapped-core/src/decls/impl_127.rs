macro_rules! deps {
    () => {
        StaticComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T > StaticComObject < T > where T : ComObjectInner , { # [doc = " Wraps `outer` in a `StaticComObject`."] pub const fn from_outer (outer : T :: Outer) -> Self { Self { outer } } }
    };
}

impl_127!();