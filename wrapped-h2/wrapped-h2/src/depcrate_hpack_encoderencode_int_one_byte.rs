// Generated macro for encode_int_one_byte (function)
macro_rules! Depcrate_hpack_encoderencode_int_one_byte {
() => {
// Module: crate::hpack::encoder
// Provides: {"encode_int_one_byte"}
// Dependencies: {}
# [doc = " Returns true if the in the int can be fully encoded in the first byte."] fn encode_int_one_byte (value : usize , prefix_bits : usize) -> bool { value < (1 << prefix_bits) - 1 }
};
}
