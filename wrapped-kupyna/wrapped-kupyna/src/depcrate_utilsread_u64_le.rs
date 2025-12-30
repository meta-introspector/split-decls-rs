// Generated macro for read_u64_le (function)
macro_rules! Depcrate_utilsread_u64_le {
() => {
// Module: crate::utils
// Provides: {"read_u64_le"}
// Dependencies: {}
# [inline (always)] pub (crate) fn read_u64_le < const N : usize , const M : usize > (src : & [u8 ; N]) -> [u64 ; M] { assert_eq ! (N , 8 * M) ; let mut res = [0 ; M] ; for (src , dst) in src . chunks_exact (8) . zip (res . iter_mut ()) { * dst = u64 :: from_le_bytes (src . try_into () . unwrap ()) ; } res }
};
}
