// Generated macro for impl_238 (impl)
macro_rules! Depcrate_frameimpl_238 {
() => {
// Module: crate::frame
// Provides: {"impl_238"}
// Dependencies: {}
impl Serialize for SerializableHeader < '_ > { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = s . serialize_struct ("header" , 2) ? ; state . serialize_field ("name" , & String :: from_utf8_lossy (self . 0 . name ())) ? ; state . serialize_field ("value" , & String :: from_utf8_lossy (self . 0 . value ())) ? ; state . end () } }
};
}
