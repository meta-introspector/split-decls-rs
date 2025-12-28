macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl DerefMut for PublicKey { # [doc = " Returns a public key as mutable bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_140!();