// Generated macro for impl_1024 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1024 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1024"}
// Dependencies: {}
impl CursorType for f32 { type Error = ParseFloatError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
};
}
