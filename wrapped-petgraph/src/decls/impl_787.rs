macro_rules! deps {
    () => {
        Somes!();
        Node!();
    };
}

macro_rules! impl_787 {
    () => {
        deps!();
        impl < N , Ix > Serialize for Somes < & [Node < Option < N > , Ix >] > where N : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq_with_length (self . 0 , self . 1 . iter () . filter_map (| node | node . weight . as_ref ()) ,) } }
    };
}

impl_787!()