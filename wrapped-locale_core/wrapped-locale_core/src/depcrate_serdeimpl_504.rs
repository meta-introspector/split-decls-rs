// Generated macro for impl_504 (impl)
macro_rules! Depcrate_serdeimpl_504 {
() => {
// Module: crate::serde
// Provides: {"impl_504"}
// Dependencies: {}
impl Serialize for LanguageIdentifier { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (& self . write_to_string ()) } }
};
}
