macro_rules! deps {
    () => {
        ReadinessVecRef!();
        ReadinessVec!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'a > Deref for ReadinessVecRef < 'a > { type Target = ReadinessVec ; fn deref (& self) -> & Self :: Target { self . inner } }
    };
}

impl_86!();