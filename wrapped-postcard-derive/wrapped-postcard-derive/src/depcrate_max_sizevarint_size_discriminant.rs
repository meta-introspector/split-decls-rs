// Generated macro for varint_size_discriminant (function)
macro_rules! Depcrate_max_sizevarint_size_discriminant {
() => {
// Module: crate::max_size
// Provides: {"varint_size_discriminant"}
// Dependencies: {}
fn varint_size_discriminant (max_n : u32) -> u32 { const BITS_PER_BYTE : u32 = 8 ; const BITS_PER_VARINT_BYTE : u32 = 7 ; let bits = core :: mem :: size_of :: < u32 > () as u32 * BITS_PER_BYTE - max_n . leading_zeros () ; let roundup_bits = bits + (BITS_PER_VARINT_BYTE - 1) ; roundup_bits / BITS_PER_VARINT_BYTE }
};
}
