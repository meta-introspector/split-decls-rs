// Generated macro for impl_837 (impl)
macro_rules! Depcrate_util_ident_string_serdeimpl_837 {
() => {
// Module: crate::util::ident_string::serde
// Provides: {"impl_837"}
// Dependencies: {}
impl serde :: Serialize for IdentString { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (self . as_str ()) } }
};
}
