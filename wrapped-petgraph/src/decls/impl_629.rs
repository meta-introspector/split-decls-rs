macro_rules! deps {
    () => {
        EdgeIndex!();
        IndexType!();
    };
}

macro_rules! impl_629 {
    () => {
        deps!();
        impl < Ix > Serialize for EdgeIndex < Ix > where Ix : IndexType + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_629!();