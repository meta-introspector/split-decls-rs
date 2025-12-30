// Generated macro for impl_36 (impl)
macro_rules! Depcrate_core_partial_versionimpl_36 {
() => {
// Module: crate::core::partial_version
// Provides: {"impl_36"}
// Dependencies: {}
impl serde :: Serialize for PartialVersion { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . collect_str (self) } }
};
}
