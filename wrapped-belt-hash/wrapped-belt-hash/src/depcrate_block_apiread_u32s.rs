// Generated macro for read_u32s (function)
macro_rules! Depcrate_block_apiread_u32s {
() => {
// Module: crate::block_api
// Provides: {"read_u32s"}
// Dependencies: {}
# [inline (always)] fn read_u32s < const N : usize > (src : & [u8]) -> [u32 ; N] { assert_eq ! (src . len () , 4 * N) ; let mut dst = [0u32 ; N] ; for (dst , src) in dst . iter_mut () . zip (src . chunks_exact (4)) { * dst = u32 :: from_le_bytes (src . try_into () . unwrap ()) ; } dst }
};
}
