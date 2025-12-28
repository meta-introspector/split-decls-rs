macro_rules! deps {
    () => {
        Map!();
        Error!();
        Value!();
        Deserializer!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'de > de :: IntoDeserializer < 'de , Error > for Map < String , Value > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_93!();