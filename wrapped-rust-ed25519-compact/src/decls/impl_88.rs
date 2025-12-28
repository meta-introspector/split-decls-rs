macro_rules! deps {
    () => {
        Noise!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl DerefMut for Noise { # [doc = " Returns the noise as mutable bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_88!();