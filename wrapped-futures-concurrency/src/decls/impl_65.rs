macro_rules! deps {
    () => {
        ReadinessArrayRef!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'a , const N : usize > DerefMut for ReadinessArrayRef < 'a , N > { fn deref_mut (& mut self) -> & mut Self :: Target { self . inner } }
    };
}

impl_65!()