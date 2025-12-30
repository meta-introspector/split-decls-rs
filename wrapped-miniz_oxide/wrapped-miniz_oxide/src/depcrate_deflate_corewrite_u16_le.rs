// Generated macro for write_u16_le (function)
macro_rules! Depcrate_deflate_corewrite_u16_le {
() => {
// Module: crate::deflate::core
// Provides: {"write_u16_le"}
// Dependencies: {}
# [cfg (test)] # [inline] fn write_u16_le (val : u16 , slice : & mut [u8] , pos : usize) { slice [pos] = val as u8 ; slice [pos + 1] = (val >> 8) as u8 ; }
};
}
