macro_rules! deps {
    () => {
        KeyPair!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl DerefMut for KeyPair { # [doc = " Returns a key pair as mutable bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . sk } }
    };
}

impl_84!();