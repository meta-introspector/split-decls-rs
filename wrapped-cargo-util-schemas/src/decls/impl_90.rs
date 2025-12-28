macro_rules! deps {
    () => {
        InheritableSemverVersion!();
        Result!();
        InheritableField!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for InheritableSemverVersion { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("SemVer version") . string (| value | match value . trim () . parse () . map_err (de :: Error :: custom) { Ok (parsed) => Ok (InheritableField :: Value (parsed)) , Err (e) => Err (e) , } ,) . map (| value | value . deserialize () . map (InheritableField :: Inherit)) . deserialize (d) } }
    };
}

impl_90!();