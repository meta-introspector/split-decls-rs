macro_rules! deps {
    () => {
        PathValue!();
        Result!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for PathValue { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { Ok (PathValue (String :: deserialize (deserializer) ? . into ())) } }
    };
}

impl_190!();