macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for Odd < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
    };
}

impl_233!();