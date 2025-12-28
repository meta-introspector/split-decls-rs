macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! from_json {
    () => {
        deps!();
        fn from_json < 'de , T , D > (deserializer : D) -> Result < T , D :: Error > where T : DeserializeOwned , D : Deserializer < 'de > , { let json = String :: deserialize (deserializer) ? ; serde_json :: from_str (& json) . map_err (de :: Error :: custom) }
    };
}

from_json!()