// Generated macro for ge_to_x25519_vartime (function)
macro_rules! Depcrate_edwards25519ge_to_x25519_vartime {
() => {
// Module: crate::edwards25519
// Provides: {"ge_to_x25519_vartime"}
// Dependencies: {}
# [cfg (feature = "x25519")] pub fn ge_to_x25519_vartime (s : & [u8 ; 32]) -> Option < [u8 ; 32] > { let p = GeP3 :: from_bytes_vartime (s) ? ; let yed = p . y ; let x_mont = (FE_ONE + yed) * ((FE_ONE - yed) . invert ()) ; Some (x_mont . to_bytes ()) }
};
}
