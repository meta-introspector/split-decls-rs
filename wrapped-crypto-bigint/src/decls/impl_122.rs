macro_rules! deps {
    () => {
        Int!();
        Encoding!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , const LIMBS : usize > Deserialize < 'de > for Int < LIMBS > where Int < LIMBS > : Encoding , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let mut buffer = Self :: ZERO . to_le_bytes () ; serdect :: array :: deserialize_hex_or_bin (buffer . as_mut () , deserializer) ? ; Ok (Self :: from_le_bytes (buffer)) } }
    };
}

impl_122!()