// Generated macro for varint_max (function)
macro_rules! Depcrate_varintvarint_max {
() => {
// Module: crate::varint
// Provides: {"varint_max"}
// Dependencies: {}
# [doc = " Returns the maximum number of bytes required to encode T."] pub const fn varint_max < T : Sized > () -> usize { const BITS_PER_BYTE : usize = 8 ; const BITS_PER_VARINT_BYTE : usize = 7 ; let bits = core :: mem :: size_of :: < T > () * BITS_PER_BYTE ; let roundup_bits = bits + (BITS_PER_VARINT_BYTE - 1) ; roundup_bits / BITS_PER_VARINT_BYTE }
};
}
