macro_rules! deps {
    () => {
        RefMut!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > Deref for RefMut < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
    };
}

impl_68!();