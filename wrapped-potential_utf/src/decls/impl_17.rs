macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `serde` Cargo feature"] # [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for PotentialCodePoint { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { if deserializer . is_human_readable () { let c = < char > :: deserialize (deserializer) ? ; Ok (PotentialCodePoint :: from_char (c)) } else { let bytes = < [u8 ; 3] > :: deserialize (deserializer) ? ; Ok (PotentialCodePoint (bytes)) } } }
    };
}

impl_17!();