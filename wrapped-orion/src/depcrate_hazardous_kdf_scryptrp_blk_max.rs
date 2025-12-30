// Generated macro for RP_BLK_MAX (const)
macro_rules! Depcrate_hazardous_kdf_scryptRP_BLK_MAX {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"RP_BLK_MAX"}
// Dependencies: {}
# [doc = " scrypt `r * 128 * p` must be less than [i32::MAX]."] pub const RP_BLK_MAX : u32 = (i32 :: MAX as u32) / 128 ;
};
}
