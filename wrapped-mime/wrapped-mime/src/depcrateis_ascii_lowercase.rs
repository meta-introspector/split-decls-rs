// Generated macro for is_ascii_lowercase (function)
macro_rules! Depcrateis_ascii_lowercase {
() => {
// Module: crate
// Provides: {"is_ascii_lowercase"}
// Dependencies: {}
# [cfg_attr (not (debug_assertions) , allow (unused))] fn is_ascii_lowercase (s : & str) -> bool { ! s . as_bytes () . iter () . any (u8 :: is_ascii_uppercase) }
};
}
