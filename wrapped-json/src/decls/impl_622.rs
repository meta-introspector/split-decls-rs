macro_rules! deps {
    () => {
        Result!();
        Error!();
        Deserializer!();
        ReferenceFromString!();
        Value!();
        RawValue!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl < 'de > DeserializeSeed < 'de > for ReferenceFromString { type Value = & 'de RawValue ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
    };
}

impl_622!()