// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'de > serde_core :: Deserialize < 'de > for Platform { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; FromStr :: from_str (& s) . map_err (serde_core :: de :: Error :: custom) } }
};
}
