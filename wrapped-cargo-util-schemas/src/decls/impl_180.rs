macro_rules! deps {
    () => {
        StringOrVec!();
        Result!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for StringOrVec { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("string or list of strings") . string (| value | Ok (StringOrVec (vec ! [value . to_owned ()]))) . seq (| value | value . deserialize () . map (StringOrVec)) . deserialize (deserializer) } }
    };
}

impl_180!()