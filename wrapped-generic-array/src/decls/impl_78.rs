macro_rules! deps {
    () => {
        Dummy!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Dummy { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Dummy) } }
    };
}

impl_78!()