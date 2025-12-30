// Generated macro for impl_505 (impl)
macro_rules! Depcrate_serdeimpl_505 {
() => {
// Module: crate::serde
// Provides: {"impl_505"}
// Dependencies: {}
impl Serialize for Locale { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (& self . write_to_string ()) } }
};
}
