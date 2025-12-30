// Generated macro for _TEST_AS_USIZE_BYTES_ROUNDED_UP_ONE_BIT_HIGH (const)
macro_rules! Depcrate_bits_TEST_AS_USIZE_BYTES_ROUNDED_UP_ONE_BIT_HIGH {
() => {
// Module: crate::bits
// Provides: {"_TEST_AS_USIZE_BYTES_ROUNDED_UP_ONE_BIT_HIGH"}
// Dependencies: {}
const _TEST_AS_USIZE_BYTES_ROUNDED_UP_ONE_BIT_HIGH : () = assert ! (BitLength :: from_bits (8192 + 1) . as_usize_bytes_rounded_up () == (8192 / 8) + 1) ;
};
}
