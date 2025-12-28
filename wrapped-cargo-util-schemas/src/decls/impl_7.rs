macro_rules! deps {
    () => {
        Result!();
        PackageIdSpec!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for PackageIdSpec { fn deserialize < D > (d : D) -> std :: result :: Result < PackageIdSpec , D :: Error > where D : de :: Deserializer < 'de > , { let string = String :: deserialize (d) ? ; PackageIdSpec :: parse (& string) . map_err (de :: Error :: custom) } }
    };
}

impl_7!();