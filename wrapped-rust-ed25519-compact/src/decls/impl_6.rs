macro_rules! deps {
    () => {
        Seed!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl DerefMut for Seed { # [doc = " Returns a seed as mutable raw bytes."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_6!();