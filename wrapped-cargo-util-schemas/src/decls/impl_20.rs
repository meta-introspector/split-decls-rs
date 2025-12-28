macro_rules! deps {
    () => {
        Result!();
        PartialVersion!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl serde :: Serialize for PartialVersion { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . collect_str (self) } }
    };
}

impl_20!();