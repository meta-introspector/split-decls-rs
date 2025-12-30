// Generated macro for deserialize (function)
macro_rules! Depcrate_serdedeserialize {
() => {
// Module: crate::serde
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Serde: Deserialize with 0x-prefix and ignore case"] pub fn deserialize < 'de , D , T > (deserializer : D) -> Result < T , D :: Error > where D : serde :: Deserializer < 'de > , T : FromIterator < u8 > , { withpfx_ignorecase :: deserialize (deserializer) }
};
}
