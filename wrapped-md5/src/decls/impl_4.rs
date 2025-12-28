macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl core :: ops :: DerefMut for Digest { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_4!()