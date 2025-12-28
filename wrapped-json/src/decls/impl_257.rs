macro_rules! deps {
    () => {
        Error!();
        Value!();
        Deserializer!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'de > IntoDeserializer < 'de , Error > for Value { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_257!();