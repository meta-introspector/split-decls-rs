macro_rules! deps {
    () => {
        Encoding!();
        Uint!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < const LIMBS : usize > Serialize for Uint < LIMBS > where Uint < LIMBS > : Encoding , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serdect :: array :: serialize_hex_lower_or_bin (& Encoding :: to_le_bytes (self) , serializer) } }
    };
}

impl_394!();