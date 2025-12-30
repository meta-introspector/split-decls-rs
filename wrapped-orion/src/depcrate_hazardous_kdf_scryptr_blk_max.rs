// Generated macro for R_BLK_MAX (const)
macro_rules! Depcrate_hazardous_kdf_scryptR_BLK_MAX {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"R_BLK_MAX"}
// Dependencies: {}
# [doc = " scrypt `r * 256` must be less than [i32::MAX]."] pub const R_BLK_MAX : u32 = (i32 :: MAX as u32) / 256 ;
};
}
