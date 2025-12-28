macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8PathBufVisitor!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Utf8PathBuf { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_string (Utf8PathBufVisitor) } }
    };
}

impl_5!()