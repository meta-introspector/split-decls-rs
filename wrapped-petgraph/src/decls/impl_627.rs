macro_rules! deps {
    () => {
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        impl < Ix > Serialize for NodeIndex < Ix > where Ix : IndexType + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_627!();