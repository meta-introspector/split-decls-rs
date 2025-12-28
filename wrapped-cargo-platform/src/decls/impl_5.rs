macro_rules! deps {
    () => {
        Platform!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'de > serde_core :: Deserialize < 'de > for Platform { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; FromStr :: from_str (& s) . map_err (serde_core :: de :: Error :: custom) } }
    };
}

impl_5!()