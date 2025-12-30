// Generated macro for is_escape (function)
macro_rules! Depcrate_readis_escape {
() => {
// Module: crate::read
// Provides: {"is_escape"}
// Dependencies: {}
fn is_escape (ch : u8 , including_control_characters : bool) -> bool { ch == b'"' || ch == b'\\' || (including_control_characters && ch < 0x20) }
};
}
