macro_rules! deps {
    () => {
        StaticComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T > StaticComObject < T > where T : ComObjectInner , { # [doc = " Gets access to the contained value."] pub const fn get (& 'static self) -> & 'static T :: Outer { & self . outer } }
    };
}

impl_128!()