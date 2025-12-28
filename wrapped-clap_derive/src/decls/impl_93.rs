macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < T > Deref for Sp < T > { type Target = T ; fn deref (& self) -> & T { & self . val } }
    };
}

impl_93!();