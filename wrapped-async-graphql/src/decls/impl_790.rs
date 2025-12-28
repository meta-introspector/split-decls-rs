macro_rules! deps {
    () => {
        MaybeUndefined!();
        Error!();
        Result!();
    };
}

macro_rules! impl_790 {
    () => {
        deps!();
        impl < 'de , T > Deserialize < 'de > for MaybeUndefined < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < MaybeUndefined < T > , D :: Error > where D : Deserializer < 'de > , { Option :: < T > :: deserialize (deserializer) . map (| value | match value { Some (value) => MaybeUndefined :: Value (value) , None => MaybeUndefined :: Null , }) } }
    };
}

impl_790!()