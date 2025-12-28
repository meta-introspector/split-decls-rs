macro_rules! deps {
    () => {
        Int!();
        Encoding!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < const LIMBS : usize > Serialize for Int < LIMBS > where Int < LIMBS > : Encoding , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serdect :: array :: serialize_hex_lower_or_bin (& Encoding :: to_le_bytes (self) , serializer) } }
    };
}

impl_123!()