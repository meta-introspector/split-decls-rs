macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Deref for PublicKey { type Target = [u8 ; PublicKey :: BYTES] ; # [doc = " Returns a public key as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_59!();