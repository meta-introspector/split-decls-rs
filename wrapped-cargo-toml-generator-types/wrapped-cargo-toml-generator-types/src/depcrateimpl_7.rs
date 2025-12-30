// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Serialize for Dependency { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self { Dependency :: Version (s) if s . is_empty () => serializer . serialize_str ("*") , Dependency :: Version (s) => serializer . serialize_str (s) , Dependency :: Table (table) => table . serialize (serializer) , } } }
};
}
