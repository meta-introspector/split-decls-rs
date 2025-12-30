// Generated macro for deserialize_from (function)
macro_rules! Depcrate_ext_dedeserialize_from {
() => {
// Module: crate::ext::de
// Provides: {"deserialize_from"}
// Dependencies: {}
# [inline] pub fn deserialize_from < 'de , T , D > (val : D) -> Result < T , Error > where T : Deserialize < 'de > , D : Deserializer < 'de , Error = Error > { Deserialize :: deserialize (val) }
};
}
