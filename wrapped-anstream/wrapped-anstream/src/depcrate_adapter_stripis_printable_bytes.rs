// Generated macro for is_printable_bytes (function)
macro_rules! Depcrate_adapter_stripis_printable_bytes {
() => {
// Module: crate::adapter::strip
// Provides: {"is_printable_bytes"}
// Dependencies: {}
# [inline] fn is_printable_bytes (action : Action , byte : u8) -> bool { const DEL : u8 = 0x7f ; (action == Action :: Print && byte != DEL) || action == Action :: BeginUtf8 || (action == Action :: Execute && byte . is_ascii_whitespace ()) }
};
}
