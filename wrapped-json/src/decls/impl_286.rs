macro_rules! deps {
    () => {
        KeyClassifier!();
        KeyClass!();
        Deserializer!();
        Error!();
        Value!();
        Result!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < 'de > DeserializeSeed < 'de > for KeyClassifier { type Value = KeyClass ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: Deserializer < 'de > , { deserializer . deserialize_str (self) } }
    };
}

impl_286!();