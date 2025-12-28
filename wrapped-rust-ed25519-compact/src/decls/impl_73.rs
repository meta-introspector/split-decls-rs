macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl DerefMut for Signature { # [doc = " Returns a signature as mutable bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_73!()