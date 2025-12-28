macro_rules! deps {
    () => {
        RefMulti!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > Deref for RefMulti < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
    };
}

impl_55!()