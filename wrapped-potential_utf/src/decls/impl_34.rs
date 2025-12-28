macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `serde` Cargo feature"] # [cfg (all (feature = "serde" , feature = "alloc"))] impl < 'de > serde_core :: Deserialize < 'de > for Box < PotentialUtf8 > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { if deserializer . is_human_readable () { let boxed_str = Box :: < str > :: deserialize (deserializer) ? ; Ok (PotentialUtf8 :: from_boxed_str (boxed_str)) } else { let boxed_bytes = Box :: < [u8] > :: deserialize (deserializer) ? ; Ok (PotentialUtf8 :: from_boxed_bytes (boxed_bytes)) } } }
    };
}

impl_34!();