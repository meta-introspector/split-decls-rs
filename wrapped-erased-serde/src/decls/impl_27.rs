macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'de , T > Sealed for erase :: Deserializer < T > where T : serde :: Deserializer < 'de > { }
    };
}

impl_27!();