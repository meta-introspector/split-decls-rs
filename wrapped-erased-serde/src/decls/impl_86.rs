macro_rules! deps {
    () => {
        Serializer!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T > sealed :: serializer :: Sealed for erase :: Serializer < T > where T : serde :: Serializer { }
    };
}

impl_86!();