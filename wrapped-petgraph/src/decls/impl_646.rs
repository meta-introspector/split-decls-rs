macro_rules! deps {
    () => {
        Graph!();
        EdgeType!();
        IndexType!();
        DeserGraph!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        # [doc = " Requires crate feature `\"serde-1\"`"] impl < 'de , N , E , Ty , Ix > Deserialize < 'de > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Deserialize < 'de > , N : Deserialize < 'de > , E : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Self :: from_deserialized (DeserGraph :: deserialize (deserializer) ?) } }
    };
}

impl_646!();