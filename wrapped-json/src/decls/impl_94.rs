macro_rules! deps {
    () => {
        Value!();
        Deserializer!();
        Error!();
        Map!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'de > de :: IntoDeserializer < 'de , Error > for & 'de Map < String , Value > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_94!();