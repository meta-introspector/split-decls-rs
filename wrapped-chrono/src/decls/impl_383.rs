macro_rules! deps {
    () => {
        Error!();
        NaiveDateTime!();
        NaiveDateTimeVisitor!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for NaiveDateTime { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (NaiveDateTimeVisitor) } }
    };
}

impl_383!()