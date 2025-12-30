// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1031 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1031"}
// Dependencies: {}
# [cfg (feature = "uuid")] impl CursorType for uuid :: Uuid { type Error = uuid :: Error ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
};
}
