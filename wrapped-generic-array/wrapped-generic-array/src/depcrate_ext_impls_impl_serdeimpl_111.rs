// Generated macro for impl_111 (impl)
macro_rules! Depcrate_ext_impls_impl_serdeimpl_111 {
() => {
// Module: crate::ext_impls::impl_serde
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Dummy { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (Dummy) } }
};
}
