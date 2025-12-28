macro_rules! deps {
    () => {
        ReadinessVecRef!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < 'a > DerefMut for ReadinessVecRef < 'a > { fn deref_mut (& mut self) -> & mut Self :: Target { self . inner } }
    };
}

impl_87!()