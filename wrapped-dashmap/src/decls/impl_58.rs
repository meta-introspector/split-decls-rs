macro_rules! deps {
    () => {
        RefMutMulti!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > Deref for RefMutMulti < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
    };
}

impl_58!();