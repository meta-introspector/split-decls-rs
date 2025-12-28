macro_rules! deps {
    () => {
        DirectDeref!();
        Strategy!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T , S : Strategy < Arc < T > > > Deref for DirectDeref < Arc < T > , S > { type Target = T ; fn deref (& self) -> & T { self . 0 . deref () . deref () } }
    };
}

impl_8!();