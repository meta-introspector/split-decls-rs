macro_rules! deps {
    () => {
        Error!();
        NaiveTime!();
        NaiveTimeVisitor!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for NaiveTime { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (NaiveTimeVisitor) } }
    };
}

impl_486!()