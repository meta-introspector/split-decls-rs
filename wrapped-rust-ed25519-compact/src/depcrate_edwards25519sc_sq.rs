// Generated macro for sc_sq (function)
macro_rules! Depcrate_edwards25519sc_sq {
() => {
// Module: crate::edwards25519
// Provides: {"sc_sq"}
// Dependencies: {}
# [cfg (feature = "blind-keys")] pub fn sc_sq (s : & [u8]) -> [u8 ; 32] { sc_mul (s , s) }
};
}
