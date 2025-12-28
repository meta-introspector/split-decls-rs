macro_rules! deps {
    () => {
        RefMutMulti!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > DerefMut for RefMutMulti < 'a , K , V > { fn deref_mut (& mut self) -> & mut V { self . value_mut () } }
    };
}

impl_59!();