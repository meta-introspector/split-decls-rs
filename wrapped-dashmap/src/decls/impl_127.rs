macro_rules! deps {
    () => {
        RefMulti!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash > Deref for RefMulti < 'a , K > { type Target = K ; fn deref (& self) -> & K { self . key () } }
    };
}

impl_127!();