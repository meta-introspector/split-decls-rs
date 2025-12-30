// Generated macro for impl_501 (impl)
macro_rules! Depcrate_wrappingimpl_501 {
() => {
// Module: crate::wrapping
// Provides: {"impl_501"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for Wrapping < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Self (T :: deserialize (deserializer) ?)) } }
};
}
