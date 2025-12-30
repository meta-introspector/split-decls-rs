// Generated macro for impl_89 (impl)
macro_rules! Depcrate_server_messageimpl_89 {
() => {
// Module: crate::server_message
// Provides: {"impl_89"}
// Dependencies: {}
impl Serialize for ErrorPayload { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . error . serialize (serializer) } }
};
}
