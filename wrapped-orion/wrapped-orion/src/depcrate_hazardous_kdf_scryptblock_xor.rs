// Generated macro for block_xor (function)
macro_rules! Depcrate_hazardous_kdf_scryptblock_xor {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"block_xor"}
// Dependencies: {}
fn block_xor (dst : & mut [u32] , src : & [u32] , n : usize) { for (i , elem) in src [.. n] . iter () . enumerate () { dst [i] ^= elem ; } }
};
}
