macro_rules! deps {
    () => {
        Suffix!();
        Error!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl serde :: Serialize for Suffix { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (match self { Self :: Kibi => "k" , Self :: Mebi => "m" , Self :: Gibi => "g" , }) } }
    };
}

impl_37!()