macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < S > DerefMut for Cache < S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } }
    };
}

impl_142!()