// Generated macro for varint_size (function)
macro_rules! Depcrate_max_sizevarint_size {
() => {
// Module: crate::max_size
// Provides: {"varint_size"}
// Dependencies: {}
# [cfg (any (feature = "heapless" , feature = "heapless-v0_8" , feature = "heapless-v0_9"))] const fn varint_size (max_n : usize) -> usize { const BITS_PER_BYTE : usize = 8 ; const BITS_PER_VARINT_BYTE : usize = 7 ; if max_n == 0 { return 1 ; } let bits = core :: mem :: size_of :: < usize > () * BITS_PER_BYTE - max_n . leading_zeros () as usize ; let roundup_bits = bits + (BITS_PER_VARINT_BYTE - 1) ; roundup_bits / BITS_PER_VARINT_BYTE }
};
}
