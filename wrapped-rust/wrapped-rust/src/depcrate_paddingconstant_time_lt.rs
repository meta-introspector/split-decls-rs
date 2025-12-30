// Generated macro for constant_time_lt (function)
macro_rules! Depcrate_paddingconstant_time_lt {
() => {
// Module: crate::padding
// Provides: {"constant_time_lt"}
// Dependencies: {}
# [doc = " This returns 0xFF if a < b else 0x00, but does so in a constant time"] # [doc = " fashion."] fn constant_time_lt (a : u8 , b : u8) -> u8 { duplicate_msb_to_all (a ^ ((a ^ b) | (a . wrapping_sub (b) ^ b))) }
};
}
