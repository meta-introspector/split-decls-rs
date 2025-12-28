macro_rules! deps {
    () => {
        StringOrBool!();
        Result!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for StringOrBool { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . bool (| b | Ok (StringOrBool :: Bool (b))) . string (| s | Ok (StringOrBool :: String (s . to_owned ()))) . deserialize (deserializer) } }
    };
}

impl_182!()