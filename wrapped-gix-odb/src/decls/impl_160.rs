macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < T > DerefMut for Proxy < T > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } }
    };
}

impl_160!()