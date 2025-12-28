macro_rules! deps {
    () => {
        Graph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        # [doc = " Requires crate feature `\"serde-1\"`"] impl < N , E , Ty , Ix > Serialize for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType + Serialize , N : Serialize , E : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . into_serializable () . serialize (serializer) } }
    };
}

impl_641!()