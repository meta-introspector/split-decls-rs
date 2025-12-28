macro_rules! deps {
    () => {
        RefMut!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > DerefMut for RefMut < 'a , K , V > { fn deref_mut (& mut self) -> & mut V { self . value_mut () } }
    };
}

impl_69!();