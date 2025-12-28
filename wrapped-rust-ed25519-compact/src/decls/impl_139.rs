macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl Deref for PublicKey { type Target = [u8 ; PublicKey :: BYTES] ; # [doc = " Returns a public key as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_139!()