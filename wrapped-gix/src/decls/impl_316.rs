macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl Deref for Buffer < '_ > { type Target = Vec < u8 > ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_316!();