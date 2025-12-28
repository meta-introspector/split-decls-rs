macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > Deref for Ref < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
    };
}

impl_64!()