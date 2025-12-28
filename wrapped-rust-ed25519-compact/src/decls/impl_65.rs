macro_rules! deps {
    () => {
        SecretKey!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl DerefMut for SecretKey { # [doc = " Returns a secret key as mutable bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_65!()