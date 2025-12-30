// Generated macro for impl_14 (impl)
macro_rules! Depcrate_serde_implsimpl_14 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_14"}
// Dependencies: {}
impl Serialize for Utf8PathBuf { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_str () . serialize (serializer) } }
};
}
