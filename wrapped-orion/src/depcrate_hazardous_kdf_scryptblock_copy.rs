// Generated macro for block_copy (function)
macro_rules! Depcrate_hazardous_kdf_scryptblock_copy {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"block_copy"}
// Dependencies: {}
fn block_copy (dst : & mut [u32] , src : & [u32] , n : usize) { dst [.. n] . copy_from_slice (& src [.. n]) ; }
};
}
