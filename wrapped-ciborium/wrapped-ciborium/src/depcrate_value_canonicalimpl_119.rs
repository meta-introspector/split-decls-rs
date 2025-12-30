// Generated macro for impl_119 (impl)
macro_rules! Depcrate_value_canonicalimpl_119 {
() => {
// Module: crate::value::canonical
// Provides: {"impl_119"}
// Dependencies: {}
impl ser :: Serialize for CanonicalValue { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . 0 . serialize (serializer) } }
};
}
