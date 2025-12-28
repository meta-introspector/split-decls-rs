macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl serde :: Serialize for Attribute { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (& self . to_string ()) } }
    };
}

impl_24!();