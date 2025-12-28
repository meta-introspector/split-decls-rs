macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < T > Deref for Proxy < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_159!();