macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! private_if_absent {
    () => {
        deps!();
        # [cfg (feature = "serde")] fn private_if_absent < 'de , D > (deserializer : D) -> Result < Data , D :: Error > where D : Deserializer < 'de > , { let option = Option :: deserialize (deserializer) ? ; Ok (option . unwrap_or (Data :: Private)) }
    };
}

private_if_absent!();