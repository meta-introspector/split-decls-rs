macro_rules! deps {
    () => {
        NonZero!();
        Zero!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > + Zero > Deserialize < 'de > for NonZero < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let value : T = T :: deserialize (deserializer) ? ; if bool :: from (value . is_zero ()) { Err (D :: Error :: invalid_value (Unexpected :: Other ("zero") , & "a non-zero value" ,)) } else { Ok (Self (value)) } } }
    };
}

impl_213!();