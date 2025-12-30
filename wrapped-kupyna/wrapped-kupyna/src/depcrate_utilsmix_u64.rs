// Generated macro for mix_u64 (function)
macro_rules! Depcrate_utilsmix_u64 {
() => {
// Module: crate::utils
// Provides: {"mix_u64"}
// Dependencies: {}
fn mix_u64 (val : & mut u64) { let mix_bytes = array :: from_fn (| i | { let a = val . to_ne_bytes () ; let b = MDS_MATRIX [i] . to_ne_bytes () ; let mut res = 0 ; for i in 0 .. 8 { res ^= multiply_gf (a [i] , b [i]) ; } res }) ; * val = u64 :: from_be_bytes (mix_bytes) ; }
};
}
