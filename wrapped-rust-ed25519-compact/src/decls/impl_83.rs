macro_rules! deps {
    () => {
        KeyPair!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Deref for KeyPair { type Target = [u8 ; KeyPair :: BYTES] ; # [doc = " Returns a key pair as bytes."] fn deref (& self) -> & Self :: Target { & self . sk } }
    };
}

impl_83!()