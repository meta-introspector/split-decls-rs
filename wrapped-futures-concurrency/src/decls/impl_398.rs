macro_rules! deps {
    () => {
        Keyed!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl < S : Stream > DerefMut for Keyed < S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . group } }
    };
}

impl_398!();