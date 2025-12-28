macro_rules! deps {
    () => {
        Limb!();
        Word!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for Limb { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Self (Word :: deserialize (deserializer) ?)) } }
    };
}

impl_170!()