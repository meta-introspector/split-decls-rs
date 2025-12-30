// Generated macro for impl_115 (impl)
macro_rules! Depcrate_typesimpl_115 {
() => {
// Module: crate::types
// Provides: {"impl_115"}
// Dependencies: {}
impl Serialize for SourceItemOrderingWithinModuleItemGroupings { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match self { SourceItemOrderingWithinModuleItemGroupings :: All => serializer . serialize_str ("all") , SourceItemOrderingWithinModuleItemGroupings :: None => serializer . serialize_str ("none") , SourceItemOrderingWithinModuleItemGroupings :: Custom (vec) => vec . serialize (serializer) , } } }
};
}
