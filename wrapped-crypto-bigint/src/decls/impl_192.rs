macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < T : ? Sized > AsRef < T > for NonZero < T > { fn as_ref (& self) -> & T { & self . 0 } }
    };
}

impl_192!();