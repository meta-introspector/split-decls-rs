macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < S > Deref for Cache < S > { type Target = S ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_141!();