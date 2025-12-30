// Generated macro for write_u64_be (function)
macro_rules! Depcrate_utilswrite_u64_be {
() => {
// Module: crate::utils
// Provides: {"write_u64_be"}
// Dependencies: {}
# [inline (always)] pub (crate) fn write_u64_be (src : & [u64] , dst : & mut [u8]) { assert_eq ! (8 * src . len () , dst . len ()) ; for (src , dst) in src . iter () . zip (dst . chunks_exact_mut (8)) { dst . copy_from_slice (& src . to_be_bytes ()) } }
};
}
