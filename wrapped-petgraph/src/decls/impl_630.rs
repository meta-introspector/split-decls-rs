macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndex!();
    };
}

macro_rules! impl_630 {
    () => {
        deps!();
        impl < 'de , Ix > Deserialize < 'de > for EdgeIndex < Ix > where Ix : IndexType + Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (EdgeIndex (Ix :: deserialize (deserializer) ?)) } }
    };
}

impl_630!()