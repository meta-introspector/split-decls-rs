// Generated macro for _TEST_AS_USIZE_BYTES_ROUNDED_UP_SEVEN_BITS_HIGH (const)
macro_rules! Depcrate_bits_TEST_AS_USIZE_BYTES_ROUNDED_UP_SEVEN_BITS_HIGH {
() => {
// Module: crate::bits
// Provides: {"_TEST_AS_USIZE_BYTES_ROUNDED_UP_SEVEN_BITS_HIGH"}
// Dependencies: {}
const _TEST_AS_USIZE_BYTES_ROUNDED_UP_SEVEN_BITS_HIGH : () = assert ! (BitLength :: from_bits (8192 + 7) . as_usize_bytes_rounded_up () == (8192 / 8) + 1) ;
};
}
