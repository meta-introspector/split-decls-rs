// Generated macro for write_u32s (function)
macro_rules! Depcrate_block_apiwrite_u32s {
() => {
// Module: crate::block_api
// Provides: {"write_u32s"}
// Dependencies: {}
# [inline (always)] fn write_u32s (src : & [u32] , dst : & mut [u8]) { assert_eq ! (4 * src . len () , dst . len ()) ; for (src , dst) in src . iter () . zip (dst . chunks_exact_mut (4)) { dst . copy_from_slice (& src . to_le_bytes ()) ; } }
};
}
