// Generated macro for xor (function)
macro_rules! Depcrate_utilsxor {
() => {
// Module: crate::utils
// Provides: {"xor"}
// Dependencies: {}
# [inline (always)] pub (crate) fn xor < const N : usize > (a : [u64 ; N] , b : [u64 ; N]) -> [u64 ; N] { let mut result = [0u64 ; N] ; for i in 0 .. N { result [i] = a [i] ^ b [i] ; } result }
};
}
