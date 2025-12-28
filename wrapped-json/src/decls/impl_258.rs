macro_rules! deps {
    () => {
        Value!();
        Deserializer!();
        Error!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'de > IntoDeserializer < 'de , Error > for & 'de Value { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_258!()