macro_rules! deps {
    () => {
        Strategy!();
        DirectDeref!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T , S : Strategy < Rc < T > > > Deref for DirectDeref < Rc < T > , S > { type Target = T ; fn deref (& self) -> & T { self . 0 . deref () . deref () } }
    };
}

impl_10!();