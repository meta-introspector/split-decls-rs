macro_rules! deps {
    () => {
        Keyed!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < F : Future > DerefMut for Keyed < F > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . group } }
    };
}

impl_210!();