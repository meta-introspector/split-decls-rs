macro_rules! deps {
    () => {
        TomlLint!();
        TomlLintLevel!();
        Result!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for TomlLint { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . string (| string | { TomlLintLevel :: deserialize (string . into_deserializer ()) . map (TomlLint :: Level) }) . map (| map | map . deserialize () . map (TomlLint :: Config)) . deserialize (deserializer) } }
    };
}

impl_171!()