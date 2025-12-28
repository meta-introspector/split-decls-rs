macro_rules! deps {
    () => {
        Result!();
        PartialVersion!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'de > serde :: Deserialize < 'de > for PartialVersion { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("SemVer version") . string (| value | value . parse () . map_err (serde :: de :: Error :: custom)) . deserialize (deserializer) } }
    };
}

impl_21!();