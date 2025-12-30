// Generated macro for private_if_absent (function)
macro_rules! Depcrateprivate_if_absent {
() => {
// Module: crate
// Provides: {"private_if_absent"}
// Dependencies: {}
# [cfg (feature = "serde")] fn private_if_absent < 'de , D > (deserializer : D) -> Result < Data , D :: Error > where D : Deserializer < 'de > , { let option = Option :: deserialize (deserializer) ? ; Ok (option . unwrap_or (Data :: Private)) }
};
}
