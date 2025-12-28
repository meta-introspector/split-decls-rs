macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl DerefMut for PublicKey { # [doc = " Returns a public key as mutable bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_60!();