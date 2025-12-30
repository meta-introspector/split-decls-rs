// Generated macro for max_of_last_byte (function)
macro_rules! Depcrate_varintmax_of_last_byte {
() => {
// Module: crate::varint
// Provides: {"max_of_last_byte"}
// Dependencies: {}
# [doc = " Returns the maximum value stored in the last encoded byte."] pub const fn max_of_last_byte < T : Sized > () -> u8 { let max_bits = core :: mem :: size_of :: < T > () * 8 ; let extra_bits = max_bits % 7 ; (1 << extra_bits) - 1 }
};
}
