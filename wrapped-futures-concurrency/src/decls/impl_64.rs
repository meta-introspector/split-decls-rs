macro_rules! deps {
    () => {
        ReadinessArrayRef!();
        ReadinessArray!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a , const N : usize > Deref for ReadinessArrayRef < 'a , N > { type Target = ReadinessArray < N > ; fn deref (& self) -> & Self :: Target { self . inner } }
    };
}

impl_64!();