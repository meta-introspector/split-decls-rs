// Generated macro for impl_204 (impl)
macro_rules! Depcrate_limbimpl_204 {
() => {
// Module: crate::limb
// Provides: {"impl_204"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for Limb { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Self (Word :: deserialize (deserializer) ?)) } }
};
}
