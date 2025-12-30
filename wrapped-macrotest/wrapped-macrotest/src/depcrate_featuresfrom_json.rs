// Generated macro for from_json (function)
macro_rules! Depcrate_featuresfrom_json {
() => {
// Module: crate::features
// Provides: {"from_json"}
// Dependencies: {}
fn from_json < 'de , T , D > (deserializer : D) -> Result < T , D :: Error > where T : DeserializeOwned , D : Deserializer < 'de > , { let json = String :: deserialize (deserializer) ? ; serde_json :: from_str (& json) . map_err (de :: Error :: custom) }
};
}
