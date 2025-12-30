// Generated macro for impl_102 (impl)
macro_rules! Depcrate_typesimpl_102 {
() => {
// Module: crate::types
// Provides: {"impl_102"}
// Dependencies: {}
impl Serialize for SourceItemOrderingModuleItemGroupings { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . groups . serialize (serializer) } }
};
}
