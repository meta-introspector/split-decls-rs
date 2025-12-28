macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl core :: ops :: Deref for Digest { type Target = [u8 ; 16] ; # [inline] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_3!();