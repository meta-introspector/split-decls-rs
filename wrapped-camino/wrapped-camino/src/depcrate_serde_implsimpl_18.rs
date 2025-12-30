// Generated macro for impl_18 (impl)
macro_rules! Depcrate_serde_implsimpl_18 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_18"}
// Dependencies: {}
impl Serialize for Utf8Path { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_str () . serialize (serializer) } }
};
}
