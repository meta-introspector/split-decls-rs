macro_rules! deps {
    () => {
        MappedRefMut!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : ? Sized > DerefMut for MappedRefMut < 'a , K , T > { fn deref_mut (& mut self) -> & mut T { self . value_mut () } }
    };
}

impl_80!();