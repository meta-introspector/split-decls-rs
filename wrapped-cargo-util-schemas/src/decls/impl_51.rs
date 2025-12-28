macro_rules! deps {
    () => {
        Result!();
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for TomlLockfileSourceId { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let s = String :: deserialize (d) ? ; Ok (TomlLockfileSourceId :: new (s) . map_err (de :: Error :: custom) ?) } }
    };
}

impl_51!();