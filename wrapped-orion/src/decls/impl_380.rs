macro_rules! deps {
    () => {
        PublicKey!();
        FieldElement!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl From < [u8 ; PUBLIC_KEY_SIZE] > for PublicKey { # [inline] fn from (bytes : [u8 ; PUBLIC_KEY_SIZE]) -> Self { Self { fe : FieldElement :: from_bytes (& bytes) , } } }
    };
}

impl_380!();