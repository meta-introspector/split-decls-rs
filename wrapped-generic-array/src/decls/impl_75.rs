macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Serialize for GenericArray < T , N > where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut tup = serializer . serialize_tuple (N :: USIZE) ? ; for el in self { tup . serialize_element (el) ? ; } tup . end () } }
    };
}

impl_75!()