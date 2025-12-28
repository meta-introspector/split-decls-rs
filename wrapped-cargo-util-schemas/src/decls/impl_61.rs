macro_rules! deps {
    () => {
        TomlLockfilePackageId!();
        Result!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for TomlLockfilePackageId { fn deserialize < D > (d : D) -> Result < TomlLockfilePackageId , D :: Error > where D : de :: Deserializer < 'de > , { String :: deserialize (d) . and_then (| string | { string . parse :: < TomlLockfilePackageId > () . map_err (de :: Error :: custom) }) } }
    };
}

impl_61!();