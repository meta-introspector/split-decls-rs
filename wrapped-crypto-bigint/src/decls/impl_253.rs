macro_rules! deps {
    () => {
        Odd!();
        Integer!();
        Zero!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > + Integer + Zero > Deserialize < 'de > for Odd < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let value : T = T :: deserialize (deserializer) ? ; Option :: < Self > :: from (Self :: new (value)) . ok_or (D :: Error :: invalid_value (Unexpected :: Other ("even") , & "a non-zero odd value" ,)) } }
    };
}

impl_253!()