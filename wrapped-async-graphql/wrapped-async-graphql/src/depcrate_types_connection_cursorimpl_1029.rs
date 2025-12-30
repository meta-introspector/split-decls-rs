// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1029 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1029"}
// Dependencies: {}
impl CursorType for ID { type Error = Infallible ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { Ok (s . to_string () . into ()) } fn encode_cursor (& self) -> String { self . to_string () } }
};
}
