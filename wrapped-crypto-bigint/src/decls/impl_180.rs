macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < T : ? Sized > NonZero < T > { # [doc = " Provides access to the contents of `NonZero` in a `const` context."] pub const fn as_ref (& self) -> & T { & self . 0 } }
    };
}

impl_180!()