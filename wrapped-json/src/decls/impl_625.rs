macro_rules! deps {
    () => {
        Error!();
        Value!();
        RawValue!();
        BoxedFromString!();
        Deserializer!();
        Result!();
    };
}

macro_rules! impl_625 {
    () => {
        deps!();
        impl < 'de > DeserializeSeed < 'de > for BoxedFromString { type Value = Box < RawValue > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
    };
}

impl_625!()