macro_rules! deps {
    () => {
        SecretKey!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Deref for SecretKey { type Target = [u8 ; SecretKey :: BYTES] ; # [doc = " Returns a secret key as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_64!();