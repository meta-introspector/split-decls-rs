// Generated macro for Aad (struct)
macro_rules! Depcrate_aeadAad {
() => {
// Module: crate::aead
// Provides: {"Aad"}
// Dependencies: {}
# [doc = " The additionally authenticated data (AAD) for an opening or sealing"] # [doc = " operation. This data is authenticated but is **not** encrypted."] # [doc = ""] # [doc = " The type `A` could be a byte slice `&[u8]`, a byte array `[u8; N]`"] # [doc = " for some constant `N`, `Vec<u8>`, etc."] pub struct Aad < A : AsRef < [u8] > > (A) ;
};
}
