macro_rules! deps {
    () => {
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < 'de , Ix > Deserialize < 'de > for NodeIndex < Ix > where Ix : IndexType + Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (NodeIndex (Ix :: deserialize (deserializer) ?)) } }
    };
}

impl_628!()