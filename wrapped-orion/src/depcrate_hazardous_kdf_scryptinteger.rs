// Generated macro for integer (function)
macro_rules! Depcrate_hazardous_kdf_scryptinteger {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"integer"}
// Dependencies: {}
fn integer (b : & [u32] , r : usize) -> u64 { let j = (2 * r - 1) * 16 ; u64 :: from (b [j]) | u64 :: from (b [j + 1]) << 32 }
};
}
