// Generated macro for Base64 (enum)
macro_rules! Depcrate_base64Base64 {
() => {
// Module: crate::base64
// Provides: {"Base64"}
// Dependencies: {}
# [doc = " Base64 encoding variants used in various MCF encodings."] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] # [non_exhaustive] pub enum Base64 { # [doc = " bcrypt encoding."] # [doc = ""] # [doc = " ```text"] # [doc = " ./         [A-Z]      [a-z]      [0-9]"] # [doc = " 0x2e-0x2f, 0x41-0x5a, 0x61-0x7a, 0x30-0x39"] # [doc = " ```"] Bcrypt , # [doc = " `crypt(3)` encoding."] # [doc = ""] # [doc = " ```text"] # [doc = " [.-9]      [A-Z]      [a-z]"] # [doc = " 0x2e-0x39, 0x41-0x5a, 0x61-0x7a"] # [doc = " ```"] Crypt , # [doc = " `crypt(3)` Base64 encoding for the following schemes:"] # [doc = " - sha1_crypt,"] # [doc = " - sha256_crypt,"] # [doc = " - sha512_crypt,"] # [doc = " - md5_crypt"] # [doc = ""] # [doc = " ```text"] # [doc = " [.-9]      [A-Z]      [a-z]"] # [doc = " 0x2e-0x39, 0x41-0x5a, 0x61-0x7a"] # [doc = " ```"] ShaCrypt , }
};
}
