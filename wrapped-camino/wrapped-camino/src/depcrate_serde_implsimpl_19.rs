// Generated macro for impl_19 (impl)
macro_rules! Depcrate_serde_implsimpl_19 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Box < Utf8Path > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Utf8PathBuf :: deserialize (deserializer) ? . into ()) } }
};
}
