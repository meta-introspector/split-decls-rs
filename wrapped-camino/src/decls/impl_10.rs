macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Serialize for Utf8Path { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_str () . serialize (serializer) } }
    };
}

impl_10!()