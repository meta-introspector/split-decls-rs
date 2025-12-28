macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Box < Utf8Path > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Utf8PathBuf :: deserialize (deserializer) ? . into ()) } }
    };
}

impl_11!()