// Generated macro for num_extra_bits_for_distance_code (function)
macro_rules! Depcrate_inflate_corenum_extra_bits_for_distance_code {
() => {
// Module: crate::inflate::core
// Provides: {"num_extra_bits_for_distance_code"}
// Dependencies: {}
# [doc = " Get the number of extra bits used for a distance code."] # [doc = " (Code numbers above `NUM_DISTANCE_CODES` will give some garbage"] # [doc = " value.)"] # [inline (always)] const fn num_extra_bits_for_distance_code (code : u8) -> u8 { let c = code >> 1 ; c . saturating_sub (1) }
};
}
