macro_rules! deps {
    () => {
        MappedRefMut!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : ? Sized > Deref for MappedRefMut < 'a , K , T > { type Target = T ; fn deref (& self) -> & T { self . value () } }
    };
}

impl_79!();