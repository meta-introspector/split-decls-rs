macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for Wrapping < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Self (T :: deserialize (deserializer) ?)) } }
    };
}

impl_437!()