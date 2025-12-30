// Generated macro for Poly1305 (struct)
macro_rules! Depcrate_poly1305Poly1305 {
() => {
// Module: crate::poly1305
// Provides: {"Poly1305"}
// Dependencies: {}
# [doc = " An instance of the Poly1305 hash algorithm."] # [doc = " - Zeroes its memory on drop."] # [doc = ""] # [doc = " This code originates from"] # [doc = " <https://github.com/DaGenix/rust-crypto/blob/master/src/poly1305.rs>."] # [doc = ""] # [doc = " It was originally a port of Andrew Moon's poly1305-donna"] # [doc = " <https://github.com/floodyberry/poly1305-donna>."] # [derive (Clone , Default)] pub struct Poly1305 { r : [u32 ; 5] , h : [u32 ; 5] , pad : [u32 ; 4] , }
};
}
