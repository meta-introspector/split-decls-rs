// Generated macro for TABLE (static)
macro_rules! Depcrate_shims_x86_gfniTABLE {
() => {
// Module: crate::shims::x86::gfni
// Provides: {"TABLE"}
// Dependencies: {}
# [doc = " A lookup table for computing the inverse byte for the inverse affine transformation."] # [doc = " See <https://www.corsix.org/content/galois-field-instructions-2021-cpus> for the"] # [doc = " definition of `gf_inv` which was used for the creation of this table."] static TABLE : [u8 ; 256] = { let mut array = [0 ; 256] ; let mut i = 1 ; while i < 256 { # [expect (clippy :: as_conversions)] let mut x = i as u8 ; let mut y = gf2p8_mul (x , x) ; x = y ; let mut j = 2 ; while j < 8 { x = gf2p8_mul (x , x) ; y = gf2p8_mul (x , y) ; j += 1 ; } array [i] = y ; i += 1 ; } array } ;
};
}
