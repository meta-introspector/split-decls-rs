// Generated macro for cursor_type_int_impl (macro)
macro_rules! Depcrate_types_connection_cursorcursor_type_int_impl {
() => {
// Module: crate::types::connection::cursor
// Provides: {"cursor_type_int_impl"}
// Dependencies: {}
macro_rules ! cursor_type_int_impl { ($ ($ t : ty) *) => { $ (impl CursorType for $ t { type Error = ParseIntError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }) * } }
};
}
