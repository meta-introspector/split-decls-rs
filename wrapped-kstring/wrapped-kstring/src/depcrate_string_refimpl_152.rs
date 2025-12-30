// Generated macro for impl_152 (impl)
macro_rules! Depcrate_string_refimpl_152 {
() => {
// Module: crate::string_ref
// Provides: {"impl_152"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de : 's , 's > serde :: Deserialize < 'de > for KStringRef < 's > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let s : & 's str = serde :: Deserialize :: deserialize (deserializer) ? ; let s = KStringRef :: from_ref (s) ; Ok (s) } }
};
}
