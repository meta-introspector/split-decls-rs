// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1026 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1026"}
// Dependencies: {}
impl CursorType for char { type Error = ParseCharError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
};
}
