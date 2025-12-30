// Generated macro for Algorithm (struct)
macro_rules! Depcrate_pbkdf2Algorithm {
() => {
// Module: crate::pbkdf2
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " A PBKDF2 algorithm."] # [doc = ""] # [doc = " `max_output_len` is computed as u64 instead of usize to prevent overflowing on 32-bit machines."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct Algorithm { algorithm : hmac :: Algorithm , max_output_len : u64 , }
};
}
