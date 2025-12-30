// Generated macro for is_utf8_continuation (function)
macro_rules! Depcrate_adapter_stripis_utf8_continuation {
() => {
// Module: crate::adapter::strip
// Provides: {"is_utf8_continuation"}
// Dependencies: {}
# [inline] fn is_utf8_continuation (b : u8) -> bool { matches ! (b , 0x80 ..= 0xbf) }
};
}
