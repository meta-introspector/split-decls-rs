macro_rules! deps {
    () => {
        ConstantDeref!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T > Deref for ConstantDeref < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
    };
}

impl_24!();