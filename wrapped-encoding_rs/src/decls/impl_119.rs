macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl Serialize for Encoding { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self . name) } }
    };
}

impl_119!()