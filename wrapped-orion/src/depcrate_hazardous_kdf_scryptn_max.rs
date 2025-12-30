// Generated macro for N_MAX (const)
macro_rules! Depcrate_hazardous_kdf_scryptN_MAX {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"N_MAX"}
// Dependencies: {}
# [doc = " scrypt `n * 128 * r` must be less than [i32::MAX]."] pub const N_MAX : u32 = (i32 :: MAX as u32) / 128 ;
};
}
