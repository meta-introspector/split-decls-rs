macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathVisitor!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a Utf8Path { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (Utf8PathVisitor) } }
    };
}

impl_9!()