macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl DerefMut for Buffer < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } }
    };
}

impl_317!()