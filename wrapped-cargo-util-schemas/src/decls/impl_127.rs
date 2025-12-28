macro_rules! deps {
    () => {
        PackageIdSpec!();
        Result!();
        ProfilePackageSpec!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for ProfilePackageSpec { fn deserialize < D > (d : D) -> Result < ProfilePackageSpec , D :: Error > where D : de :: Deserializer < 'de > , { let string = String :: deserialize (d) ? ; if string == "*" { Ok (ProfilePackageSpec :: All) } else { PackageIdSpec :: parse (& string) . map_err (de :: Error :: custom) . map (ProfilePackageSpec :: Spec) } } }
    };
}

impl_127!();