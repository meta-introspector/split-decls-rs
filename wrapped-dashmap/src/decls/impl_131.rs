macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash > Deref for Ref < 'a , K > { type Target = K ; fn deref (& self) -> & K { self . key () } }
    };
}

impl_131!();