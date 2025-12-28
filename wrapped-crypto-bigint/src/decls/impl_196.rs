macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for NonZero < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
    };
}

impl_196!();