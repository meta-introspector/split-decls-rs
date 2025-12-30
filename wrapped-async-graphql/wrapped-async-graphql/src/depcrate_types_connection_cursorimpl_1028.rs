// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1028 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1028"}
// Dependencies: {}
impl CursorType for String { type Error = Infallible ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { Ok (s . to_string ()) } fn encode_cursor (& self) -> String { self . clone () } }
};
}
