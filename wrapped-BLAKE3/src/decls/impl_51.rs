macro_rules! deps {
    () => {
        CVWords!();
        Mode!();
        Hash!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'a > Mode < 'a > { fn key_words (& self) -> CVWords { match self { Mode :: Hash => * IV , Mode :: KeyedHash (key) => crate :: platform :: words_from_le_bytes_32 (key) , Mode :: DeriveKeyMaterial (cx_key) => crate :: platform :: words_from_le_bytes_32 (cx_key) , } } fn flags_byte (& self) -> u8 { match self { Mode :: Hash => 0 , Mode :: KeyedHash (_) => crate :: KEYED_HASH , Mode :: DeriveKeyMaterial (_) => crate :: DERIVE_KEY_MATERIAL , } } }
    };
}

impl_51!()