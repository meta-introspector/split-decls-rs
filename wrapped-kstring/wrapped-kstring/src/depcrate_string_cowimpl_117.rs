// Generated macro for impl_117 (impl)
macro_rules! Depcrate_string_cowimpl_117 {
() => {
// Module: crate::string_cow
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , B : crate :: backend :: HeapStr > serde :: Deserialize < 'de > for KStringCowBase < '_ , B > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { KStringBase :: deserialize (deserializer) . map (| s | s . into ()) } }
};
}
