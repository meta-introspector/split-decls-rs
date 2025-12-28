macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        StableGraph!();
    };
}

macro_rules! impl_794 {
    () => {
        deps!();
        # [doc = " Requires crate feature `\"serde-1\"`"] impl < N , E , Ty , Ix > Serialize for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Serialize , N : Serialize , E : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . into_serializable () . serialize (serializer) } }
    };
}

impl_794!();