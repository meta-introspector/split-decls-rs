macro_rules! deps {
    () => {
        Error!();
        Deserializer!();
        Out!();
        DeserializeSeed!();
        Result!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut (dyn DeserializeSeed < 'de > + '_) { type Value = Out ; fn deserialize < D > (self , deserializer : D) -> Result < Out , D :: Error > where D : serde :: Deserializer < 'de > , { let mut erased = erase :: Deserializer :: new (deserializer) ; self . erased_deserialize_seed (& mut erased) . map_err (unerase) } }
    };
}

impl_32!()