macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        RawValue!();
    };
}

macro_rules! impl_633 {
    () => {
        deps!();
        impl < 'de > IntoDeserializer < 'de , Error > for & 'de RawValue { type Deserializer = & 'de RawValue ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_633!()