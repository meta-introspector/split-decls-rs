macro_rules! deps {
    () => {
        Odd!();
        NonZero!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < T : ? Sized > AsRef < NonZero < T > > for Odd < T > { fn as_ref (& self) -> & NonZero < T > { self . as_nz_ref () } }
    };
}

impl_230!()