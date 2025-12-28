macro_rules! deps {
    () => {
        PathValue!();
        Result!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl ser :: Serialize for PathValue { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_189!()