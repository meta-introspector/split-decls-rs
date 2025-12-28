macro_rules! deps {
    () => {
        NonZero!();
        Odd!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < T : ? Sized > AsRef < NonZero < T > > for Odd < T > { fn as_ref (& self) -> & NonZero < T > { self . as_nz_ref () } }
    };
}

impl_230!();