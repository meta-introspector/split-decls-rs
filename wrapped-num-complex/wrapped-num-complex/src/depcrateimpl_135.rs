// Generated macro for impl_135 (impl)
macro_rules! Depcrateimpl_135 {
() => {
// Module: crate
// Provides: {"impl_135"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T > serde :: Deserialize < 'de > for Complex < T > where T : serde :: Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let (re , im) = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (Self :: new (re , im)) } }
};
}
