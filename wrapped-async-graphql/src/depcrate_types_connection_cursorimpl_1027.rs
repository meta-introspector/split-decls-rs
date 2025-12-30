// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1027 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1027"}
// Dependencies: {}
impl CursorType for bool { type Error = ParseBoolError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
};
}
