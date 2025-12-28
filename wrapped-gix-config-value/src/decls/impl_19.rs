macro_rules! deps {
    () => {
        Error!();
        Name!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl serde :: Serialize for Name { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (& self . to_string ()) } }
    };
}

impl_19!();