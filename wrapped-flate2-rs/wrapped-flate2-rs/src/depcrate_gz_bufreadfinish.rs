// Generated macro for finish (function)
macro_rules! Depcrate_gz_bufreadfinish {
() => {
// Module: crate::gz::bufread
// Provides: {"finish"}
// Dependencies: {}
# [inline] fn finish (buf : & [u8 ; 8]) -> (u32 , u32) { let crc = (buf [0] as u32) | ((buf [1] as u32) << 8) | ((buf [2] as u32) << 16) | ((buf [3] as u32) << 24) ; let amt = (buf [4] as u32) | ((buf [5] as u32) << 8) | ((buf [6] as u32) << 16) | ((buf [7] as u32) << 24) ; (crc , amt) }
};
}
