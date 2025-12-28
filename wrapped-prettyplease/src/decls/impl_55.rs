macro_rules! deps {
    () => {
        IteratorItem!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < T > Deref for IteratorItem < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . value } }
    };
}

impl_55!()