// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_types_connection_cursorimpl_1030 {
() => {
// Module: crate::types::connection::cursor
// Provides: {"impl_1030"}
// Dependencies: {}
# [cfg (feature = "chrono")] impl CursorType for chrono :: DateTime < chrono :: Utc > { type Error = chrono :: ParseError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { Ok (chrono :: DateTime :: parse_from_rfc3339 (s) ? . with_timezone :: < chrono :: Utc > (& chrono :: Utc { })) } fn encode_cursor (& self) -> String { self . to_rfc3339_opts (chrono :: SecondsFormat :: Micros , true) } }
};
}
