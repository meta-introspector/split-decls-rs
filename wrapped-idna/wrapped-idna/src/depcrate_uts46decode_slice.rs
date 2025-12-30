// Generated macro for decode_slice (function)
macro_rules! Depcrate_uts46decode_slice {
() => {
// Module: crate::uts46
// Provides: {"decode_slice"}
// Dependencies: {}
fn decode_slice (slice : & StringTableSlice) -> & 'static str { let lo = slice . byte_start_lo as usize ; let hi = slice . byte_start_hi as usize ; let start = (hi << 8) | lo ; let len = slice . byte_len as usize ; & STRING_TABLE [start .. (start + len)] }
};
}
