macro_rules! deps {
    () => {
        Integer!();
        Error!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl serde :: Serialize for Integer { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if let Some (suffix) = self . suffix { serializer . serialize_i64 (self . value << suffix . bitwise_offset ()) } else { serializer . serialize_i64 (self . value) } } }
    };
}

impl_30!();