macro_rules! deps {
    () => {
        Node!();
        NodeIndex!();
        Holes!();
        IndexType!();
    };
}

macro_rules! impl_789 {
    () => {
        deps!();
        impl < N , Ix > Serialize for Holes < & [Node < Option < N > , Ix >] > where Ix : Serialize + IndexType , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq_with_length (self . 0 , self . 1 . iter () . enumerate () . filter_map (| (i , node) | { if node . weight . is_none () { Some (NodeIndex :: < Ix > :: new (i)) } else { None } }) ,) } }
    };
}

impl_789!()