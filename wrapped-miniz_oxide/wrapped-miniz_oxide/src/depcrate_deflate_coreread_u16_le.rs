// Generated macro for read_u16_le (function)
macro_rules! Depcrate_deflate_coreread_u16_le {
() => {
// Module: crate::deflate::core
// Provides: {"read_u16_le"}
// Dependencies: {}
# [inline (always)] const fn read_u16_le < const N : usize > (slice : & [u8 ; N] , pos : usize) -> u16 { slice [pos] as u16 | ((slice [pos + 1] as u16) << 8) }
};
}
