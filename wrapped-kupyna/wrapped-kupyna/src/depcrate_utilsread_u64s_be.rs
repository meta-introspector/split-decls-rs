// Generated macro for read_u64s_be (function)
macro_rules! Depcrate_utilsread_u64s_be {
() => {
// Module: crate::utils
// Provides: {"read_u64s_be"}
// Dependencies: {}
# [inline (always)] pub (crate) fn read_u64s_be < const N : usize , const M : usize > (block : & [u8 ; N]) -> [u64 ; M] { array :: from_fn (| i | { let chunk = block [8 * i ..] [.. 8] . try_into () . unwrap () ; u64 :: from_be_bytes (chunk) }) }
};
}
