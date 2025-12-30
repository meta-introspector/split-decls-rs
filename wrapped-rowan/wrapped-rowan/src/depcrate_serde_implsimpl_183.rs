// Generated macro for impl_183 (impl)
macro_rules! Depcrate_serde_implsimpl_183 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_183"}
// Dependencies: {}
impl < T : fmt :: Display > Serialize for SerDisplay < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (& self . 0) } }
};
}
