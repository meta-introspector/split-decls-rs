macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_916 {
    () => {
        deps!();
        impl < T > Deref for Ptr < '_ , T > { type Target = T ; fn deref (& self) -> & T { self . 0 } }
    };
}

impl_916!()