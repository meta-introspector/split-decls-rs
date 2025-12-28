macro_rules! deps {
    () => {
        FieldElement!();
        PublicKey!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl PartialEq < & [u8] > for PublicKey { fn eq (& self , other : & & [u8]) -> bool { if other . len () != PUBLIC_KEY_SIZE { return false ; } let other : [u8 ; 32] = (* other) . try_into () . unwrap () ; self . fe == FieldElement :: from_bytes (& other) } }
    };
}

impl_379!();