macro_rules! deps {
    () => {
        StableGraph!();
        IndexType!();
        EdgeType!();
        DeserStableGraph!();
    };
}

macro_rules! impl_796 {
    () => {
        deps!();
        # [doc = " Requires crate feature `\"serde-1\"`"] impl < 'de , N , E , Ty , Ix > Deserialize < 'de > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Deserialize < 'de > , N : Deserialize < 'de > , E : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Self :: from_deserialized (DeserStableGraph :: deserialize (deserializer) ?) } }
    };
}

impl_796!()