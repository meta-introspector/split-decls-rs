// Generated macro for rand_array (function)
macro_rules! Depcrate_randrand_array {
() => {
// Module: crate::rand
// Provides: {"rand_array"}
// Dependencies: {}
# [doc = " Returns an array of random bytes."] pub fn rand_array < const N : usize > () -> [u8 ; N] { unsafe { with_output_array (| out , out_len | { let ret = bssl_sys :: RAND_bytes (out , out_len) ; debug_assert ! (ret == 1) ; }) } }
};
}
