macro_rules! deps {
    () => {
        VecStringOrBool!();
        Result!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for VecStringOrBool { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("a boolean or vector of strings") . bool (| value | Ok (VecStringOrBool :: Bool (value))) . seq (| value | value . deserialize () . map (VecStringOrBool :: VecString)) . deserialize (deserializer) } }
    };
}

impl_186!();