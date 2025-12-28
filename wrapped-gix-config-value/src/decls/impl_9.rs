macro_rules! deps {
    () => {
        Boolean!();
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl serde :: Serialize for Boolean { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_bool (self . 0) } }
    };
}

impl_9!()