// Generated macro for impl_509 (impl)
macro_rules! Depcrate_wrappingimpl_509 {
() => {
// Module: crate::wrapping
// Provides: {"impl_509"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for Wrapping < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Self (T :: deserialize (deserializer) ?)) } }
};
}
