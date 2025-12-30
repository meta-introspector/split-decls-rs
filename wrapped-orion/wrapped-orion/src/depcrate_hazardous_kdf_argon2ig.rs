// Generated macro for g (function)
macro_rules! Depcrate_hazardous_kdf_argon2ig {
() => {
// Module: crate::hazardous::kdf::argon2i
// Provides: {"g"}
// Dependencies: {}
# [doc = " BLAKE2 G with 64-bit multiplications."] fn g (a : & mut u64 , b : & mut u64 , c : & mut u64 , d : & mut u64) { * a = lower_mult_add (* a , * b) ; * d = (* d ^ * a) . rotate_right (32) ; * c = lower_mult_add (* c , * d) ; * b = (* b ^ * c) . rotate_right (24) ; * a = lower_mult_add (* a , * b) ; * d = (* d ^ * a) . rotate_right (16) ; * c = lower_mult_add (* c , * d) ; * b = (* b ^ * c) . rotate_right (63) ; }
};
}
