// Generated macro for update_slow (function)
macro_rules! Depcrate_baselineupdate_slow {
() => {
// Module: crate::baseline
// Provides: {"update_slow"}
// Dependencies: {}
pub (crate) fn update_slow (prev : u32 , buf : & [u8]) -> u32 { let mut crc = ! prev ; for & byte in buf . iter () { crc = CRC32_TABLE [0] [((crc as u8) ^ byte) as usize] ^ (crc >> 8) ; } ! crc }
};
}
